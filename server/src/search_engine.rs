use crate::file_contents::FileContents;
use crate::gen::searchium::{
    FileContentsQueryType, FileContentsSearchHit, FileContentsSearchRequest, FileContentsSearchRootResult
};
use crate::gen::searchium::{FileExtract, FileContentsSpan}; // TODO: remove and use internal types?

use core::fmt;
use grep::matcher::Matcher;
use grep::regex::{RegexMatcher, RegexMatcherBuilder};
use grep::searcher::{Searcher, SearcherBuilder, Sink};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

#[derive(Error, Debug)]
pub enum SearchEngineError {
    #[error("Error building query")]
    BuildError(#[from] grep::regex::Error),
}

// TODO: Move these enums to protobuf definitions
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MatchCase {
    No,
    Yes,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MatchWholeWord {
    No,
    Yes,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum QueryStyle {
    Literal,  // No interpretation of special characters
    Wildcard, // Simple wildcards like * and ?
    Regex,    // Full regex
}

#[derive(Clone, Copy)]
struct MatcherParams {
    match_case: MatchCase,
    whole_word: MatchWholeWord,
    style: QueryStyle,
}

impl Default for MatcherParams {
    fn default() -> Self {
        MatcherParams {
            match_case: MatchCase::No,
            whole_word: MatchWholeWord::No,
            style: QueryStyle::Literal,
        }
    }
}

pub fn get_file_extracts(
    contents: &FileContents,
    spans: &[FileContentsSpan],
    max_extract_len: u32,
) -> Vec<FileExtract> {
    let (line_offsets, contents_len) = calculate_line_offsets(contents); // TODO: is this worth caching?
    spans
        .iter()
        .map(|span| {
            // clamp so that we always return the match even if it's longer than the requested max extract
            let max_extract_len = max_extract_len.max(span.length_bytes) as usize;
            let span_start = span.offset_bytes as usize;
            let span_end = (span.offset_bytes + span.length_bytes) as usize;
            let line_span = find_line_span(&line_offsets, contents_len, span_start);
            let line_slice = contents
                .get_slice(line_span.offset, line_span.offset + line_span.length)
                .unwrap();
            // TODO: Handle utf16
            let line_end = if line_slice.ends_with(&[b'\r', b'\n']) {
                line_span.offset + line_span.length - 2
            } else {
                line_span.offset + line_span.length - 1
            };

            // TODO: strip whitespace from start/end before clamping size
            let extract_start = span_start
                .min((span_end + 5).saturating_sub(max_extract_len))
                .max(line_span.offset);
            let extract_end = span_end.max(extract_start + max_extract_len).min(line_end);

            // TODO: respect character boundaries in utf8/16
            let text = contents.get_text(extract_start, extract_end);
            let offset = extract_start;
            let length = extract_end - extract_start;
            let line_number = line_span.line_number;
            FileExtract {
                full_text: text,
                extract_offset_bytes: offset as u32,
                extract_length_bytes: length as u32,
                line_number: line_number as u32,
            }
        })
        .collect()
}

// TODO: Change result to not come from protobuf defs
pub fn search_files_contents(
    root_path: &Path,
    files: &HashMap<PathBuf, FileContents>,
    query: &FileContentsSearchRequest,
    cancel: CancellationToken,
) -> Result<FileContentsSearchRootResult, SearchEngineError> {
    let matcher = create_matcher_from_query(
        &query.query_string,
        MatcherParams {
            match_case: MatchCase::from(query.match_case),
            whole_word: MatchWholeWord::from(query.match_whole_word),
            // TODO: Better proto def to avoid this casting
            style: if query.query_type == (FileContentsQueryType::Plain as i32) {
                QueryStyle::Literal
            }
            else if query.query_type == (FileContentsQueryType::Regex as i32) {
                QueryStyle::Regex
            }
            else if query.query_type == (FileContentsQueryType::Wildcard as i32) {
                QueryStyle::Wildcard
            }
            else {
                QueryStyle::Literal
            }
        }
    )?;
    let searcher = SearcherBuilder::new().build();
    let mut hits: Vec<_> = files
        .par_iter()
        .map_with(
            (searcher, matcher),
            |(searcher, matcher), (path, contents)| -> Option<FileContentsSearchHit> {
                let match_spans: Vec<FileContentsSpan> = search_file_contents(contents, searcher, matcher);
                if match_spans.is_empty() {
                    None
                } else {
                    let file_relative_path = path
                        .strip_prefix(root_path)
                        .expect("file path not relative to root")
                        .to_string_lossy()
                        .to_string();
                    let response = FileContentsSearchHit {
                        file_relative_path,
                        match_spans,
                    };
                    Some(response)
                }
            },
        )
        .filter_map(|r| r)
        .take_any_while(|_| !cancel.is_cancelled())
        // TODO: Cap number of results not number of files - maybe change format and flat_map over spans?
        // TODO: sort results
        .take_any(query.max_results as usize)
        .collect();
    // TODO: Do we need a better sort than lexical for paths?
    hits.par_sort_unstable_by(|h1, h2| h1.file_relative_path.cmp(&h2.file_relative_path));
    Ok(FileContentsSearchRootResult {
        root_path: root_path.to_string_lossy().to_string(),
        hits,
    })
}

struct SearchSink<'a, M : Matcher> {
    buffer: &'a [u8],
    spans: Vec<FileContentsSpan>,
    matcher: &'a M,
}

#[derive(Error, Debug)]
enum SinkError
{
    #[error("grep error: {0}")]
    GrepError(String),
}

impl grep::searcher::SinkError for SinkError {
    fn error_message<T: std::fmt::Display>(message: T) -> Self {
        Self::GrepError(message.to_string())
    }
}

impl<'a, M : Matcher> Sink for SearchSink<'a, M> {
    type Error = SinkError;

    fn matched(
        &mut self,
        _searcher: &Searcher,
        mat: &grep::searcher::SinkMatch<'_>,
    ) -> Result<bool, Self::Error> {
        let line_bytes = mat.bytes();
        // bytes is a sub-slice of self.buffer, return offset within file by pointer arithmetic
        // TODO: take line info from mat and store it so we don't need to recalculate it later
        let line_offset = unsafe {
            let line_start = line_bytes.as_ptr();
            let buffer_start = self.buffer.as_ptr();
            line_start.offset_from(buffer_start) as usize
        };
        let matched = self.matcher.find(line_bytes)
            .map_err(|e| SinkError::GrepError(e.to_string()))?.unwrap();
        let match_offset = line_offset + matched.start();
        let match_len = matched.len();
        self.spans.push(FileContentsSpan {
            offset_bytes: match_offset as u32,
            length_bytes: match_len as u32,
        });
        Ok(true)
    }
}

// TODO: May not need to use searcher as we aren't reading from files
fn search_file_contents<M: Matcher>(
    contents: &FileContents,
    searcher: &mut Searcher,
    matcher: &M,
) -> Vec<FileContentsSpan> {
    match contents {
        FileContents::Ascii(bytes) | FileContents::Utf8(bytes) => {
            let bytes = &bytes[..];
            let mut sink = SearchSink {
                buffer: bytes,
                spans: Vec::new(),
                matcher
            };
            if let Err(e) = searcher.search_slice(matcher, bytes, &mut sink) {
                panic!("Error searching slice: {:?}", e);
            }
            sink.spans
        }
        FileContents::Utf16(_) => unimplemented!(),
        FileContents::Binary(_) => Vec::new(),
    }
}

fn calculate_line_offsets(contents: &FileContents) -> (Vec<usize>, usize) {
    match contents {
        FileContents::Ascii(vec) | FileContents::Utf8(vec) => {
            let mut offsets = vec![0];
            let mut index = 0;
            while let Some(o) = memchr::memchr(b'\n', &vec[index..]) {
                index += o + 1;
                offsets.push(index);
            }
            (offsets, vec.len())
        }
        FileContents::Utf16(_vec) => {
            todo!();
        }
        FileContents::Binary(_) => (vec![], 0),
    }
}

#[derive(PartialEq, Eq, Debug)]
struct LineSpan {
    offset: usize,
    length: usize,
    line_number: usize,
}

fn find_line_span(line_offsets: &[usize], contents_len: usize, offset: usize) -> LineSpan {
    let line_start_index = match line_offsets.binary_search(&offset) {
        Ok(exact_index) => exact_index,
        Err(insert_index) => insert_index - 1,
    };

    // Returned line number is 0-indexed
    let line_number = line_start_index;
    let offset = line_offsets[line_start_index];
    let length = *line_offsets
        .get(line_start_index + 1)
        .unwrap_or(&contents_len)
        - offset;
    LineSpan {
        offset,
        length,
        line_number,
    }
}

const WHOLE_WORD_REGEX_PREFIX: &'static str = r"(?:\b|\s|^)(";
const WHOLE_WORD_REGEX_SUFFIX: &'static str = r")(?:\b|\s\$)";

fn build_wildcard_matcher(
    builder: RegexMatcherBuilder,
    query_string: &str,
    whole_word: MatchWholeWord,
) -> Result<RegexMatcher, grep::regex::Error> {
    let mut translated = String::new();
    let len = query_string
        .matches(|c| c == '*' || c == '?')
        .map(|m| match m {
            "*" => 1,
            "?" => 0, // doesn't increase size
            _ => 0,
        })
        .sum::<usize>()
        + WHOLE_WORD_REGEX_PREFIX.len()
        + WHOLE_WORD_REGEX_SUFFIX.len()
        + query_string.len();
    translated.reserve(len);
    if whole_word == MatchWholeWord::Yes {
        translated.push_str(&WHOLE_WORD_REGEX_PREFIX);
    }
    let mut last = 0 as usize;
    for (i, m) in query_string.match_indices(|c| c == '*' || c == '?') {
        if last != i {
            translated.push_str(&regex::escape(&query_string[last..i]));
        }
        last = i + m.len();
        match m {
            "*" => translated.push_str(".*"),
            "?" => translated.push('.'),
            _ => {
                panic!("unexpected match")
            }
        }
    }
    if last != query_string.len() {
        translated.push_str(&regex::escape(&query_string[last..]));
    }
    if whole_word == MatchWholeWord::Yes {
        translated.push_str(&WHOLE_WORD_REGEX_SUFFIX);
    }
    builder.build(&translated)
}

fn build_literal_matcher(
    mut builder: RegexMatcherBuilder,
    query_string: &str,
    whole_word: MatchWholeWord,
) -> Result<RegexMatcher, grep::regex::Error> {
    match whole_word {
        MatchWholeWord::No => {
            // Can use builder's fixed_strings mode because we don't need to surround with regex to match the word boundaries
            builder.fixed_strings(true).build(query_string)
        }
        MatchWholeWord::Yes => {
            // Rather than using the grep crate's word feature, explicitly allow the matched query character to be adjacent to:
            // * a non-word charater, if the matched character is a word character (\b)
            // * a word character, if the matched character is a non-word character (\b)
            // * whitespace, even if the matched character is a non-word character and thus would not match with \b (\s)
            // * beginning or end of haystack (usually line) regardless of whether matche character is work or not (^ and $)
            // Explicitly add a capturing group around the original query so it can be extracted
            let transformed_query = format!(
                r"{}({}){}",
                WHOLE_WORD_REGEX_PREFIX,
                regex::escape(query_string),
                WHOLE_WORD_REGEX_SUFFIX
            );
            builder.build(&transformed_query)
        }
    }
}

// TODO: Make wildcards such as * and ? a user setting
// TODO: Refactor input to omit unnecessary fields
fn create_matcher_from_query(
    query_string: &str,
    params: MatcherParams,
) -> Result<impl Matcher + Clone + fmt::Debug, SearchEngineError> {
    let mut builder = RegexMatcherBuilder::new();
    builder
        .case_insensitive(params.match_case == MatchCase::No) // TODO: Consider 'smart' case option
        ;
    let matcher = match params.style {
        QueryStyle::Regex => builder
            .word(params.whole_word == MatchWholeWord::Yes)
            .build(&query_string), // TODO: Desired whole-word handling
        QueryStyle::Wildcard => build_wildcard_matcher(builder, query_string, params.whole_word),
        QueryStyle::Literal => build_literal_matcher(builder, query_string, params.whole_word),
    }?;
    Ok(matcher)
}

impl From<bool> for MatchCase {
    fn from(value: bool) -> Self {
        if value {
            MatchCase::Yes
        } else {
            MatchCase::No
        }
    }
}

impl From<bool> for MatchWholeWord {
    fn from(value: bool) -> Self {
        if value {
            MatchWholeWord::Yes
        } else {
            MatchWholeWord::No
        }
    }
}

#[cfg(test)]
mod tests {
    use grep::matcher::Captures;
    use lazy_static::lazy_static;
    use tracing::{span, Level};
    use tracing_subscriber::{prelude::*, EnvFilter};

    use super::*;
    use crate::gen::searchium::FileContentsSpan;

    impl FileContentsSpan {
        fn new(offset_bytes: u32, length_bytes: u32) -> Self {
            Self {
                offset_bytes,
                length_bytes,
            }
        }
    }

    lazy_static! {
        static ref init_tracing: () = do_init_tracing();
    }

    fn do_init_tracing() {
        let fmt = tracing_subscriber::fmt::layer()
            .compact()
            .with_file(false)
            .with_line_number(false);
        tracing_subscriber::registry()
            .with(fmt)
            .with(EnvFilter::from_env("SEARCHIUM_LOG_TEST"))
            .try_init()
            .ok();
    }

    // TODO: Tests for non-ascii search

    #[test]
    fn test_ascii_line_offsets() {
        let lines = [
            "Lorem ipsum dolor sit amet, consectetur",
            "adipiscing elit, sed do eiusmod tempor",
            "incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,",
            "quis nostrud exercitation",
            "ullamco laboris nisi ut aliquip ex ea commodo consequat.",
            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
            "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia",
            "deserunt mollit anim id est laborum.",
        ];
        let string = lines.join("\n");
        let vec = string.as_bytes().to_vec();
        let file_contents = FileContents::Ascii(vec.clone());
        let (offsets, _contents_len) = calculate_line_offsets(&file_contents);
        assert_eq!(lines.len(), offsets.len());
        for (window, line) in offsets.windows(2).zip(lines.into_iter()) {
            let start = window[0];
            let end = window[1] - 1; // 1 for line break
            let sub = &vec[start..end];
            let sub = String::from_utf8_lossy(sub);
            assert_eq!(sub, line);
        }
    }

    #[test]
    fn test_line_span() {
        let offsets = vec![0, 12, 20, 28];
        let len = 40;
        // Exact start of line
        assert_eq!(
            find_line_span(&offsets, len, 12),
            LineSpan {
                offset: 12,
                length: 8,
                line_number: 1
            },
            "Line 1 span incorrect"
        );
        // First line
        assert_eq!(
            find_line_span(&offsets, len, 4),
            LineSpan {
                offset: 0,
                length: 12,
                line_number: 0
            },
            "Line 0 span incorrect"
        );
        // Last line
        assert_eq!(
            find_line_span(&offsets, len, 30),
            LineSpan {
                offset: 28,
                length: 12,
                line_number: 3
            },
            "Last line span incorrect"
        );
    }

    #[test]
    fn test_get_extracts() {
        let lines = [
            "Lorem ipsum dolor sit amet, consectetur",
            "adipiscing elit, sed do eiusmod tempor",
            "incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,",
            "quis nostrud exercitation",
            "ullamco laboris nisi ut aliquip ex ea commodo consequat.",
            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
            "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia",
            "deserunt mollit anim id est laborum.",
        ];
        let string = lines.join("\n");
        let vec = string.as_bytes().to_vec();
        let file_contents = FileContents::Ascii(vec.clone());
        let spans = vec![
            FileContentsSpan::new(12, 5),                                                     // dolor
            FileContentsSpan::new(lines[0].len() as u32 + 1 + 11, 4),                         // elit
            FileContentsSpan::new(lines[0].len() as u32 + lines[1].len() as u32 + 2 + 56, 5), // minim
        ];
        let extracts = get_file_extracts(&file_contents, &spans, 40);
        assert_eq!(extracts.len(), spans.len());
        assert_eq!(
            extracts[0],
            FileExtract {
                full_text: lines[0].to_owned(),
                extract_offset_bytes: 0,
                extract_length_bytes: lines[0].len() as u32,
                line_number: 0,
            }
        );
        assert_eq!(
            extracts[1],
            FileExtract {
                full_text: lines[1].to_owned(),
                extract_offset_bytes: lines[0].len() as u32 + 1,
                extract_length_bytes: lines[1].len() as u32,
                line_number: 1,
            }
        );
        // Line too long for max extract length
        // TODO: Define length as bytes, code units or code points
        assert!(extracts[2].full_text.len() <= 40);
        let extract_pos = lines[2].find(&extracts[2].full_text);
        assert!(extract_pos.is_some());
        assert_eq!(extracts[2].line_number, 2);
        assert_eq!(
            extracts[2].extract_offset_bytes,
            (lines[0].len() + lines[1].len() + 2 + extract_pos.unwrap_or(0)) as u32
        );
    }

    // TODO: tests for wildcard search, regex search, word boundary and case settings

    // Test the given query string, built with the given matcher params, against the haystack
    // The haystack may mark the expected match start and end if any with backticks (`)
    // Return Ok(Option<(usize, usize)>) if the expected match was found
    // Return Err if the expected match was not found, or an unexpected match was found
    fn test_query(
        params: MatcherParams,
        query: &str,
        haystack: &str,
    ) -> Result<Option<(usize, usize)>, String> {
        let matcher = create_matcher_from_query(query, params)
            .map_err(|e| format!("Failed to create matcher: {}", e))?;
        let indices: Vec<usize> = haystack.match_indices("`").map(|t| t.0).collect();
        let (h, e) = if indices.len() == 0 {
            (haystack.to_owned(), None)
        } else if indices.len() == 2 {
            (
                haystack.replace("`", ""),
                Some((indices[0], indices[1] - 1)),
            )
        } else {
            panic!("Unexpected number of backticks in haystack {}", haystack);
        };

        // One capture implies the default whole-match capture
        let maybe_match = if matcher.capture_count() > 1 {
            // We have modified the original query so we need to look for a capture that represents the actual result the user wants
            let mut c = matcher.new_captures().map_err(|e| {
                format!(
                    "Unexpected error {} getting new capture set for matcher from query {}",
                    e, query
                )
            })?;
            if matcher.captures(h.as_bytes(), &mut c).map_err(|e| {
                format!(
                    "Unexpected error {} capturing {} against {}",
                    e, query, haystack
                )
            })? {
                c.get(1) // Return the first explicit capture
            } else {
                None
            }
        } else {
            matcher.find(h.as_bytes()).map_err(|e| {
                format!(
                    "Unexpected error {} matching {} against {}",
                    e, query, haystack
                )
            })?
        };
        let actual = maybe_match.map(|m| (m.start(), m.end()));
        if actual == e {
            Ok(e)
        } else {
            match e {
                Some(_) => Err(format!(
                    "Query `{}` against haystack `{}`. Expected {:?} but got {:?}",
                    query, h, e, actual
                )),
                None => Err(format!(
                    "Unexpected match for query `{}` against haystack `{}`: {:?}",
                    query, haystack, actual
                )),
            }
        }
    }

    // Test case-insensitive, non-whole-world, literal matches
    #[test]
    fn test_plain_match() -> Result<(), String> {
        let _ = *init_tracing;
        let _span = span!(Level::INFO, "test_plain_match").entered();
        let params = MatcherParams::default();

        // No special characters
        test_query(params, "foo", "bar")?;
        test_query(params, "foo", "fo")?;
        test_query(params, "foo", "fo o")?;
        test_query(params, "foo", "`foo`")?;
        test_query(params, "foo", "`foo` bar")?;
        test_query(params, "foo", "`foo`bar")?;

        // Special regex character matched literally
        test_query(params, ".txt", "blah")?;
        test_query(params, ".txt", "log:txt")?;
        test_query(params, ".txt", "log`.txt`")?;
        test_query(params, ".txt", "`.txt`")?;
        test_query(params, ".txt", "`.txt`db")?;
        test_query(params, ".txt", "foo`.txt`db")?;

        // Special wildcard/regex character matched literally
        test_query(params, "void*", "voi")?;
        test_query(params, "void*", "voii")?;
        test_query(params, "void*", "void")?;
        test_query(params, "void*", "`void*`")?;
        test_query(params, "void*", "`void*` data")?;

        Ok(())
    }

    #[test]
    fn test_whole_word_match() -> Result<(), String> {
        let _ = *init_tracing;
        let _span = span!(Level::INFO, "test_whole_word_match").entered();
        let params = MatcherParams {
            whole_word: MatchWholeWord::Yes,
            ..MatcherParams::default()
        };

        // Queries containing basic word characters
        test_query(params, "txt", "blah")?;
        test_query(params, "txt", "ftxt")?;
        test_query(params, "txt", "txte")?;
        test_query(params, "txt", "otxte")?;
        test_query(params, "txt", " `txt` ")?; // space is treated as a word boundary on both sides
        test_query(params, "txt", "`txt`")?;
        test_query(params, "txt", ".`txt`")?; // . is treated as a word boundary
        test_query(params, "txt", "`txt`.")?; // . is treated as a word boundary
        test_query(params, "txt", ".`txt`.")?; // . is treated as a word boundary on both sides

        // Queries which themselves contain word boundaries
        test_query(params, ".txt", "blah")?;
        test_query(params, ".txt", "`.txt`")?;
        test_query(params, ".txt", "foo`.txt`")?; // o and . are different class so treated as a boundary
        test_query(params, ".txt", "..txt")?; // Adjacent .. are the same class so not treated as a boundary
        test_query(params, "two words", "`two words`")?;
        test_query(params, "two words", "there are more than `two words`")?;
        test_query(params, "two words", "there are more than `two words`.")?;
        test_query(
            params,
            "two words",
            "there are more than `two words` in this sentence",
        )?;

        // e.g. searching for implementation of a function, but not another function which starts with the same prefix
        test_query(params, "::MemberName", "ClassName`::MemberName`")?;
        test_query(params, "::MemberName", "ClassName::MemberNameBlah")?;

        Ok(())
    }

    #[test]
    fn test_wildcard_match() -> Result<(), String> {
        let _ = *init_tracing;
        let _span = span!(Level::INFO, "test_wildcard_match").entered();
        let params = MatcherParams {
            style: QueryStyle::Wildcard,
            ..MatcherParams::default()
        };

        // One character wildcard
        test_query(params, "foo?bar", "foobar")?;
        test_query(params, "foo?bar", "`fooobar`")?;
        test_query(params, "foo?bar", "`fooxbar`")?;
        test_query(params, "foo?bar", "foxobar")?;

        // Any number of characters wildcard
        test_query(params, "foo*bar", "`foobar`")?;
        test_query(params, "foo*bar", "`foobazbar`")?;
        test_query(params, "foo*bar", "fobar")?;
        test_query(params, "foo*bar", "foobr")?;
        test_query(params, "foo*bar", "fofbar")?;
        test_query(params, "foo*bar", "f`foobar`")?;
        test_query(params, "foo*bar", "`foobar`baz")?;
        test_query(params, "foo*bar", "bing`foobar`baz")?;

        Ok(())
    }

    #[test]
    fn test_wildcard_whole_word_match() -> Result<(), String> {
        let _ = *init_tracing;
        let _span = span!(Level::INFO, "test_wildcard_whole_word_match").entered();
        let params = MatcherParams {
            style: QueryStyle::Wildcard,
            whole_word: MatchWholeWord::Yes,
            ..MatcherParams::default()
        };

        // beginning/end of haystack
        test_query(params, "foo*bar", "`foobar`")?;
        test_query(params, "foo*bar", "`foobazbar`")?;

        // bounded by other word characters
        test_query(params, "foo*bar", "bingfoobarbaz")?;

        // spaces
        test_query(params, "foo*bar", "pre `foobar` post")?;
        test_query(params, "foo*bar", "pre `fooxbar` post")?;

        Ok(())
    }

    #[test]
    fn test_search_file_contents() -> Result<(), String> {
        let mut searcher = SearcherBuilder::new().build();
        let params = MatcherParams::default();
        let query = "int";
        let matcher = create_matcher_from_query(query, params)
            .map_err(|e| format!("Failed to create matcher: {}", e))?;
        
        let text = "void do_something(int param);";
        let contents = &FileContents::Utf8(text.as_bytes().to_vec());
        let spans = search_file_contents(contents, &mut searcher, &matcher);
        assert_eq!(1, spans.len());
        assert_eq!(FileContentsSpan::new(18, 3), spans[0]);
        Ok(())
    }
}
