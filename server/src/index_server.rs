use crate::file_contents::load_files;
use crate::file_contents::FileContents;
use crate::fs_filter::*;
use crate::fs_state::*;
use crate::search_engine::{get_file_extracts, search_files_contents};
use crate::searchium;

use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fmt::Debug, time::SystemTime};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use tonic::Status;
use tracing::{event, info_span, Level};

type TonicResult<T> = Result<T, tonic::Status>;

// Commands for index state to update
#[derive(Debug)]
pub enum Command {
    RegisterFolder(
        searchium::FolderRegisterRequest,
        broadcast::Sender<TonicResult<searchium::IndexUpdate>>,
    ),
    UnregisterFolder(searchium::FolderUnregisterRequest),
    FilePathSearch(
        searchium::FilePathSearchRequest,
        oneshot::Sender<searchium::FilePathSearchResponse>,
    ),
    FileContentsSearch(
        searchium::FileContentsSearchRequest,
        mpsc::Sender<TonicResult<searchium::FileContentsSearchResponse>>,
    ),
    GetFileExtracts(
        searchium::FileExtractsRequest,
        oneshot::Sender<TonicResult<searchium::FileExtractsResponse>>,
    ),
}

pub struct IndexServer {
    command_rx: mpsc::Receiver<Command>,
    // TODO: move roots into search_engine.rs
    roots: Vec<Root>,
    // TODO: move contents into roots
    contents: Vec<HashMap<PathBuf, FileContents>>,
}

impl IndexServer {
    pub fn new(command_rx: mpsc::Receiver<Command>) -> Self {
        IndexServer {
            command_rx,
            roots: Vec::new(),
            contents: Vec::new(),
        }
    }

    pub fn run(self) {
        tokio::spawn(async move { self.run_internal().await });
    }
}

impl std::fmt::Debug for IndexServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

impl searchium::IndexUpdate {
    fn scan_start() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(searchium::index_update::Type::FilesystemScanStart(
                searchium::index_update::FilesystemScanStart::default(),
            )),
        }
    }
    fn scan_end() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(searchium::index_update::Type::FilesystemScanEnd(
                searchium::index_update::FilesystemScanEnd::default(),
            )),
        }
    }
    fn loaded() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(searchium::index_update::Type::FileContentsLoaded(
                searchium::index_update::FileContentsLoaded::default(),
            )),
        }
    }
}

// TODO: Setup directory watcher to feed update commands back into index state
impl IndexServer {
    async fn run_internal(mut self) {
        event!(Level::INFO, "Starting index state command loop");
        while let Some(c) = self.command_rx.recv().await {
            // TODO: Consider how to handle parallelism here i.e. getting more commands while working on an indexing operation
            self.execute_command(c).await;
        }
        event!(Level::INFO, "Leaving index state command loop");
    }

    // TODO: Return result type and instrument
    async fn execute_command(&mut self, c: Command) {
        match c {
            // TODO: Handle overlapping folders
            Command::RegisterFolder(params, tx) => {
                let span = info_span!("RegisterFolder");
                let _ = span.enter();

                tx.send(Ok(searchium::IndexUpdate::scan_start())).ok(); // TODO: Handle error
                event!(Level::DEBUG, ?params.ignore_file_globs, ?params.path, "Constructing path filter");
                let scan_filter =
                    PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_file_globs);
                event!(Level::INFO, ignore_search_globs = ?params.ignore_search_globs, "Constructing search filter");
                let search_filter =
                    PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_search_globs);
                // TODO: Dupe detection, check for different ignore globs
                let new_root = scan_filesystem(Path::new(&params.path), &scan_filter, &search_filter);
                tx.send(Ok(searchium::IndexUpdate::scan_end())).ok();

                // Load the contents of all discovered files in the new root into memory
                let contents = load_files(new_root.searchable_files());
                
                // TODO: Accept results and filter out failures/binary files 
                let contents: HashMap<PathBuf, _> = new_root.searchable_files()
                    .iter()
                    .map(|p| p.to_path_buf())
                    .zip(contents)
                    .collect();
                tx.send(Ok(searchium::IndexUpdate::loaded())).ok();
                event!(Level::INFO, "Finished loading file contents");
                // Add the root to make it available for searches
                self.roots.push(new_root);
                self.contents.push(contents);
            }
            Command::UnregisterFolder(_params) => {
                // TODO: Remove root directory if it exists
            }
            Command::FilePathSearch(params, tx) => {
                let span = info_span!("FilePathSearch");
                let _ = span.enter();
                let mut results = Vec::new();
                let start = std::time::Instant::now();
                let fragments: Vec<_> = params.query.split_whitespace().collect();

                event!(Level::DEBUG, ?fragments);

                // TODO: Unit test for search query
                for root in &self.roots {
                    // TODO: limit number of results per parallel task
                    results.par_extend(
                        root.all_files()
                            .par_iter()
                            .filter_map(|file| match match_file_path(file, &fragments) {
                                Some(s) => {
                                    event!(Level::DEBUG, ?s, "match");
                                    Some(s)
                                }
                                None => None,
                            })
                            .take_any(params.max_results as usize),
                    );
                }
                results.truncate(params.max_results as usize);
                event!(Level::DEBUG, "total results {}", results.len());
                let duration =
                    prost_types::Duration::try_from(std::time::Instant::now() - start).ok();
                tx.send(searchium::FilePathSearchResponse { results, duration })
                    .ok();
            }
            Command::FileContentsSearch(params, tx) => {
                let token = CancellationToken::new();
                for (root, contents) in self.roots.iter().zip(self.contents.iter()) {
                    search_files_contents(
                        root.directory().path(),
                        contents,
                        &params,
                        |r| {
                            tx.blocking_send(Ok(r)).ok();
                        },
                        token.clone(),
                    );
                }
            }
            Command::GetFileExtracts(request, tx) => {
                tx.send({
                    let path = PathBuf::from(request.file_path);
                    if !path.is_absolute() {
                        Err(Status::invalid_argument(
                            "File argument must be an absolute path",
                        ))
                    } else if let Some(contents) =
                        self.contents.iter().find_map(|map| map.get(&path))
                    {
                        let file_extracts =
                            get_file_extracts(contents, &request.spans, request.max_extract_length);
                        Ok(searchium::FileExtractsResponse {
                            file_path: path.to_string_lossy().to_string(),
                            file_extracts,
                        })
                    } else {
                        Err(Status::invalid_argument("File not found"))
                    }
                })
                .ok();
            }
        }
    }
}

fn match_file_path(file: &Path, fragments: &[&str]) -> Option<String> {
    file.to_str().and_then(|s| {
        fragments
            .iter()
            .all(|f| s.contains(f))
            .then(|| s.to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_fragment_match() {
        let file = "C:\\code\\projects\\foo\\src\\module.cpp";
        let path = PathBuf::from(file);
        let fragments: Vec<_> = "foo".split_whitespace().collect();
        let m = match_file_path(&path, &fragments);
        assert_eq!(m.as_ref().map(|s| s.as_str()), Some(file));
    }

    #[test]
    fn test_two_fragment_match() {
        let file = "C:\\code\\projects\\foo\\src\\module.cpp";
        let path = PathBuf::from(file);
        let fragments: Vec<_> = "foo cpp".split_whitespace().collect();
        let m = match_file_path(&path, &fragments);
        assert_eq!(m.as_ref().map(|s| s.as_str()), Some(file));
    }
}
