use futures::{stream::FuturesOrdered, StreamExt};
use std::{
    fs::File,
    io::{Read, Seek},
    path::{Path, PathBuf},
    pin::pin,
};

#[allow(dead_code)]
#[derive(Clone)]
pub enum FileContents {
    Ascii(Vec<u8>),
    Utf8(Vec<u8>),
    Utf16(Vec<u8>),
    Binary(usize), // TODO: remove?
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

pub struct FileLoadEvent {
    pub count: usize,
    pub path: PathBuf,
}

pub async fn load_files<Paths, PathIter>(
    paths: Paths,
    events_tx: tokio::sync::mpsc::Sender<FileLoadEvent>,
) -> Vec<FileContents>
where
    Paths: IntoIterator<Item = PathBuf, IntoIter = PathIter>,
    PathIter: ExactSizeIterator<Item = PathBuf>,
{
    let mut path_iter = paths.into_iter();
    let num_paths = path_iter.len();
    let mut res = Vec::new();
    let max_tasks = 200;
    let mut update_count = 0;
    let handle = tokio::runtime::Handle::current();
    // TODO: Consider FuturesOrdered
    let mut futures = pin!(FuturesOrdered::new());
    while res.len() < num_paths {
        while futures.len() < max_tasks {
            if let Some(path) = path_iter.next() {
                let task = handle.spawn_blocking(move || {
                    let path_ref = path.as_ref();
                    let contents = read_file_contents(path_ref);
                    (path, contents)
                });
                futures.push_back(task);
            } else {
                break;
            }
        }

        match futures.next().await {
            None => {
                continue;
            }
            Some(Err(_)) => {
                panic!();
            }
            Some(Ok((path, contents))) => {
                res.push(contents.unwrap()); // TODO: propagate error
                update_count += 1;
                if update_count >= 100 {
                    events_tx
                        .try_send(FileLoadEvent {
                            count: update_count,
                            path,
                        })
                        .ok();
                    update_count = 0;
                }
            }
        }
    }
    res
}

fn read_file_contents(path: &Path) -> Result<FileContents, std::io::Error> {
    // let mut contents = Vec::new();
    let file = File::open(path)?;
    // TODO: Load in chunks for large files
    // let size = file.read_to_end(&mut contents)?;
    // TODO: Check for BOM of utf-16/32
    // Discard utf8 BOM
    // TODO: Optimize to avoid shifting large file
    // if size >= 3 && contents[..3] == [0xEF, 0xBB, 0xBF] {
    //     contents = Vec::from(&contents[3..]);
    // }
    classify_file(file)
}

const CLASSIFY_SLICE_COUNT: u64 = 50;
const CLASSIFY_SLICE_SIZE: u64 = 4 * 1024;
const CLASSIFY_TOTAL_SAMPLE_BYTES: u64 = CLASSIFY_SLICE_COUNT * CLASSIFY_SLICE_SIZE;

// TODO: Move constants out so they can be shared with tests
// TODO: Try and classify files as utf-16
fn classify_file(mut file: impl Read + Seek) -> Result<FileContents, std::io::Error> {
    file.seek(std::io::SeekFrom::End(0))?;
    let total_len = file.stream_position()?;
    file.seek(std::io::SeekFrom::Start(0))?;

    let (classification, has_bom) = if CLASSIFY_TOTAL_SAMPLE_BYTES >= total_len {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        if slice_starts_with_bom(&bytes) {
            (classify_slice(&bytes[3..]), true)
        } else {
            (classify_slice(&bytes), false)
        }
    } else {
        let chunk_size = total_len / CLASSIFY_SLICE_COUNT;
        let mut has_bom = false;
        let classification = (0..CLASSIFY_SLICE_COUNT)
            .map(|slice_index| {
                let mut slice = vec![0u8; CLASSIFY_SLICE_SIZE as usize]; // TODO: usize confusion
                file.seek(std::io::SeekFrom::Start(slice_index * chunk_size))
                    .unwrap(); // TODO handle error
                file.read_exact(&mut slice).unwrap();
                if slice_index == 0 {
                    has_bom = slice_starts_with_bom(&slice);
                    classify_slice(if has_bom { &slice[3..] } else { &slice })
                } else {
                    classify_slice(&slice)
                }
            })
            .fold(Classification::default(), Classification::combine);
        (classification, has_bom)
    };

    let total_classified =
        classification.other_count + classification.utf8_count + classification.ascii_count;
    let other_ratio = classification.other_count as f64 / total_classified as f64;

    if other_ratio > 0.1 {
        return Ok(FileContents::Binary(total_len as usize));
    }

    let mut contents = Vec::new();
    file.seek(std::io::SeekFrom::Start(if has_bom { 3 } else { 0 }))
        .unwrap();
    file.read_to_end(&mut contents).unwrap();
    Ok(if classification.utf8_count == 0 {
        FileContents::Ascii(contents)
    } else {
        FileContents::Utf8(contents)
    })
}

fn slice_starts_with_bom(bytes: &[u8]) -> bool {
    return bytes.len() >= 3 && bytes[..3] == [0xEF, 0xBB, 0xBF];
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
    while !slice.is_empty() {
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
            n => {
                c.utf8_count += 1;
                slice = &slice[n..];
            }
        }
    }
    c
}

fn is_ascii(c: u8) -> bool {
    (b' '..=b'~').contains(&c) || c == b'\t' || c == b'\r' || c == b'\n'
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
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_classify_small_ascii() {
        let data = Vec::from("abcdefghijklmnop");
        assert!(data.len() < 200 * 1024);
        let data = Cursor::new(data);
        let file = classify_file(data);
        match file {
            Ok(FileContents::Ascii(_)) => {}
            _ => assert!(false, "File not classified as ascii"),
        }
    }

    #[test]
    fn test_classify_200k_ascii() {
        let str = "abcdefghijklmnopqrstuvwxyz0123456789";
        let vec: Vec<u8> = str.bytes().cycle().take(200 * 1024).collect();
        assert_eq!(vec.len(), 200 * 1024);
        let data = Cursor::new(vec);
        let file = classify_file(data);
        match file {
            Ok(FileContents::Ascii(_)) => {}
            _ => assert!(false, "File not classified as ascii"),
        }
    }

    #[test]
    fn test_classify_uneven_ascii() {
        let str = "abcdefghijklmnopqrstuvwxyz0123456789";
        // This size should ensure that there is a small leftover chunk when dividing into 50 chunks
        let vec: Vec<u8> = str.bytes().cycle().take(200 * 1024 + 4).collect();
        let data = Cursor::new(vec);
        let file = classify_file(data);
        match file {
            Ok(FileContents::Ascii(_)) => {}
            _ => assert!(false, "File not classified as ascii"),
        }
    }

    #[test]
    fn test_classify_small_binary() {
        let data: Vec<u8> = std::iter::repeat(0xFF as u8).take(1024).collect();
        let datalen = data.len();
        assert!(data.len() < 200 * 1024);
        let data = Cursor::new(data);
        let file = classify_file(data);
        match file {
            Ok(FileContents::Binary(size)) => {
                assert_eq!(size, datalen);
            }
            _ => assert!(false, "File not classified as binary"),
        }
    }
}
