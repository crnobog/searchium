use rayon::prelude::*;
use std::{fs::File, io::Read, path::Path};

#[allow(dead_code)]
pub enum FileContents {
    Ascii(Vec<u8>),
    Utf8(Vec<u8>),
    Utf16(Vec<u8>),
    Binary,                // TODO: remove?
    Error(std::io::Error), // TODO: remove
}

impl FileContents {
    pub fn get_text(&self, start: usize, end: usize) -> String {
        match self {
            FileContents::Ascii(vec) | FileContents::Utf8(vec) => {
                String::from_utf8_lossy(&vec[start..end]).to_string()
            }
            _ => {
                unimplemented!("TODO")
            }
        }
    }
}

pub fn load_files<Paths, PathItem>(paths: Paths) -> Vec<FileContents>
where
    Paths: for<'a> rayon::iter::IntoParallelIterator<Item = PathItem>,
    PathItem: AsRef<Path>,
{
    let mut res = Vec::new();
    res.par_extend(
        paths
            .into_par_iter()
            .map(|p| read_file_contents(p.as_ref()))
            .map(|c| c.unwrap_or_else(|e| FileContents::Error(e))),
    );
    res
}

fn read_file_contents(path: &Path) -> Result<FileContents, std::io::Error> {
    let mut contents = Vec::new();
    let mut file = File::open(path)?;
    // TODO: Load in chunks for large files
    let size = file.read_to_end(&mut contents)?;
    // TODO: Check for BOM of utf-16/32
    // Discard utf8 BOM
    // TODO: Optimize to avoid shifting large file
    if size >= 3 && &contents[..3] == &[0xEF, 0xBB, 0xBF] {
        contents = Vec::from(&contents[3..]);
    }
    Ok(classify_file(contents))
}

// TODO: Move constants out so they can be shared with tests
fn classify_file(contents: Vec<u8>) -> FileContents {
    let slice_count = 50;
    let slice_size = 4 * 1024;
    let total_sample = slice_count * slice_size;
    let classification = if total_sample > contents.len() {
        classify_slice(&contents)
    } else {
        let chunk_size = contents.len() / slice_count;
        contents
            .chunks_exact(chunk_size)
            .map(|chunk| {
                assert!(chunk.len() >= slice_size);
                classify_slice(&chunk[0..slice_size])
            })
            .fold(Classification::default(), Classification::combine)
    };
    let total = classification.other_count + classification.utf8_count + classification.ascii_count;
    let other_ratio = classification.other_count as f64 / total as f64;
    if other_ratio > 0.1 {
        FileContents::Binary
    } else if classification.utf8_count == 0 {
        FileContents::Ascii(contents)
    } else {
        FileContents::Utf8(contents)
    }
}

#[derive(Default)]
struct Classification {
    ascii_count: usize,
    utf8_count: usize,
    other_count: usize,
}

impl Classification {
    fn combine(self, other: Self) -> Self {
        Classification {
            ascii_count: self.ascii_count + other.ascii_count,
            utf8_count: self.utf8_count + other.utf8_count,
            other_count: self.other_count + other.other_count,
        }
    }
}

fn classify_slice(mut slice: &[u8]) -> Classification {
    let mut c = Classification::default();
    while slice.len() > 0 {
        if is_ascii(slice[0]) {
            c.ascii_count += 1;
            slice = &slice[1..];
            continue;
        }
        match utf8_rune_length(slice) {
            0 => {
                c.other_count += 1;
                slice = &slice[1..];
            }
            n @ _ => {
                c.utf8_count += 1;
                slice = &slice[n..];
            }
        }
    }
    c
}

fn is_ascii(c: u8) -> bool {
    (c >= b' ' && c <= b'~') || c == b'\t' || c == b'\r' || c == b'\n'
}

fn utf8_rune_length(cs: &[u8]) -> usize {
    let len = cs[0].leading_ones() as usize;
    match len {
        4 | 3 | 2 if (cs.len() >= len && cs[0..len].iter().all(|u| u.leading_ones() == 1)) => len,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_small_ascii() {
        let data = Vec::from("abcdefghijklmnop");
        assert!(data.len() < 200 * 1024);
        let file = classify_file(data);
        match file {
            FileContents::Ascii(_) => {}
            _ => assert!(false, "File not classified as ascii"),
        }
    }

    #[test]
    fn test_classify_200k_ascii() {
        let str = "abcdefghijklmnopqrstuvwxyz0123456789";
        let vec: Vec<u8> = str.bytes().cycle().take(200 * 1024).collect();
        assert_eq!(vec.len(), 200 * 1024);
        let file = classify_file(vec);
        match file {
            FileContents::Ascii(_) => {}
            _ => assert!(false, "File not classified as ascii"),
        }
    }

    #[test]
    fn test_classify_uneven_ascii() {
        let str = "abcdefghijklmnopqrstuvwxyz0123456789";
        // This size should ensure that there is a small leftover chunk when dividing into 50 chunks
        let vec: Vec<u8> = str.bytes().cycle().take(200 * 1024 + 4).collect();
        let file = classify_file(vec);
        match file {
            FileContents::Ascii(_) => {}
            _ => assert!(false, "File not classified as ascii"),
        }
    }

    #[test]
    fn test_classify_small_binary() {
        let data: Vec<u8> = std::iter::repeat(0xFF as u8).take(1024).collect();
        assert!(data.len() < 200 * 1024);
        let file = classify_file(data);
        match file {
            FileContents::Binary => {}
            _ => assert!(false, "File not classified as binary"),
        }
    }
}
