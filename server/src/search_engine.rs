use crate::file_contents::FileContents;
use crate::searchium;

use memchr::memmem;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio_util::sync::CancellationToken;

pub fn get_file_extracts(
    contents: &FileContents,
    spans: &[searchium::Span],
    max_extract_len: u32,
) -> Vec<searchium::FileExtract> {
    let (line_offsets, contents_len) = calculate_line_offsets(contents); // TODO: is this worth caching?
    spans
        .into_iter()
        .map(|span| {
            // clamp so that we always return the match even if it's longer than the requested max extract
            let max_extract_len = max_extract_len.max(span.length_bytes) as usize;
            let span_start = span.offset_bytes as usize;
            let span_end = (span.offset_bytes + span.length_bytes) as usize;
            let line_span = find_line_span(&line_offsets, contents_len, span_start);
            let line_end = line_span.offset + line_span.length;

            let extract_start = span_start
                .min(span_end + 5 - max_extract_len)
                .max(line_span.offset);
            let extract_end = span_end.max(extract_start + max_extract_len).min(line_end);

            // TODO: respect character boundaries in utf8/16
            let text = contents.get_text(extract_start, extract_end);
            let offset = extract_start;
            let length = extract_end;
            let line_number = line_span.line_number;
            let column_number = extract_start - line_span.offset + 1;
            searchium::FileExtract {
                text,
                offset: offset as u32,
                length: length as u32,
                line_number: line_number as u32,
                column_number: column_number as u32,
            }
        })
        .collect()
}

pub fn search_files_contents(
    root_path: &Path,
    files: &HashMap<PathBuf, FileContents>,
    query: &searchium::FileContentsSearchRequest,
    tx: impl Fn(searchium::FileContentsSearchResponse) -> () + Sync + Send,
    cancel: CancellationToken,
) {
    let finder = memmem::Finder::new(&query.query_string);
    let root_path_string = root_path.to_string_lossy().to_string();
    files
        .par_iter()
        .try_for_each_with(finder, |finder, (path, contents)| -> Result<(), ()> {
            if cancel.is_cancelled() {
                return Err(());
            }

            let spans: Vec<searchium::Span> = search_file_contents(contents, finder);
            if spans.len() != 0 {
                let file_relative_path = path
                    .strip_prefix(root_path)
                    .expect("file path not relative to root")
                    .to_string_lossy()
                    .to_string();
                let response = searchium::FileContentsSearchResponse {
                    root_path: root_path_string.clone(),
                    file_relative_path,
                    spans,
                };
                tx(response);
            }
            Ok(())
        })
        .ok();
}

fn search_file_contents(contents: &FileContents, finder: &memmem::Finder) -> Vec<searchium::Span> {
    let length = finder.needle().len();
    match contents {
        FileContents::Ascii(bytes) | FileContents::Utf8(bytes) => finder
            .find_iter(&bytes[..])
            .map(|start| searchium::Span {
                offset_bytes: start as u32,
                length_bytes: length as u32,
            })
            .collect(),
        _ => Vec::new(),
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
        FileContents::Binary => (vec![], 0),
        FileContents::Error(_) => {
            todo!("Remove this case");
        }
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

    let line_number = line_start_index + 1;
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

#[cfg(test)]
mod tests {
    use super::*;

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
                length: 8
            },
            "Line 2 span incorrect"
        );
        // First line
        assert_eq!(
            find_line_span(&offsets, len, 4),
            LineSpan {
                offset: 0,
                length: 12
            },
            "Line 1 span incorrect"
        );
        // Last line
        assert_eq!(
            find_line_span(&offsets, len, 30),
            LineSpan {
                offset: 28,
                length: 12
            },
            "Last line span incorrect"
        );
    }
}
