use std::ffi::OsStr;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use memory_stats::memory_stats;
use rayon::prelude::*;
use tokio::sync::{mpsc, watch};
use tokio_util::sync::CancellationToken;
use tracing::{event, info_span, Level};

use crate::file_contents::{load_files, FileContents, FileLoadEvent};
use crate::fs_filter::PathGlobFilter;
use crate::{gen::*, index::match_file_path};

use super::{CommandError, CommandResult, Configuration, Root};

pub fn new(status_tx : watch::Sender<StatusResponse>) -> State {
    State {
        configuration: Configuration::default(),
        roots: Vec::new(),
        contents: Vec::new(),
        status_tx, 
    }
}

pub struct State {
    configuration: Configuration,
    // TODO: move roots into search_engine.rs
    roots: Vec<Root>,
    // TODO: move contents into roots
    // TODO: document which files should be present in contents
    contents: Vec<HashMap<PathBuf, FileContents>>,
    // TODO: Ideally don't want any message passing stuff to be directly in IndexState
    status_tx : watch::Sender<StatusResponse>,
}

impl State {
    pub fn get_num_searchable_files(&self) -> u64 {
        self.roots
            .iter()
            .map(|r| r.searchable_files().len())
            .sum::<usize>() as u64
    }

    pub fn send_status(&self, state: crate::gen::IndexState) -> Option<()> {
        let stats = memory_stats()?;
        let mem_usage = stats.physical_mem as u64;
        let num_searchable_files = self.get_num_searchable_files();
        self.status_tx
            .send(StatusResponse {
                state: state.into(),
                mem_usage,
                num_searchable_files,
            }).ok()
    }

    pub fn set_configuration(&mut self, params: ConfigurationRequest) -> CommandResult<()> {
        if params.concurrent_file_reads != 0 {
            self.configuration.concurrent_file_reads = params.concurrent_file_reads;
        }
        if params.max_file_size != 0 {
            self.configuration.max_file_size = params.max_file_size;
        }
        Ok(())
    }

    pub fn search_file_contents(
        &self,
        params: FileContentsSearchRequest,
        token: CancellationToken,
    ) -> CommandResult<FileContentsSearchResponse> {
        Ok(FileContentsSearchResponse {
            roots: self
                .roots
                .iter()
                .zip(self.contents.iter())
                .map(|(root, contents)| {
                    crate::search_engine::search_files_contents(
                        root.directory().path(),
                        contents,
                        &params,
                        token.clone(),
                    )
                })
                .collect(),
        })
    }

    pub fn search_file_paths(
        &mut self,
        params: FilePathSearchRequest,
    ) -> CommandResult<FilePathSearchResponse> {
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
        let duration = prost_types::Duration::try_from(std::time::Instant::now() - start).ok();
        Ok(FilePathSearchResponse { results, duration })
    }

    pub fn get_file_extracts(
        &self,
        request: FileExtractsRequest,
    ) -> CommandResult<FileExtractsResponse> {
        let path = PathBuf::from(request.file_path);
        if !path.is_absolute() {
            Err(CommandError::InvalidArgument(
                path.to_string_lossy().to_string(),
            ))
        } else if let Some(contents) = self.contents.iter().find_map(|map| map.get(&path)) {
            let file_extracts = crate::search_engine::get_file_extracts(
                contents,
                &request.spans,
                request.max_extract_length,
            );
            Ok(FileExtractsResponse {
                file_path: path.to_string_lossy().to_string(),
                file_extracts,
            })
        } else {
            Err(CommandError::NotFound(path.to_string_lossy().to_string()))
        }
    }

    pub fn get_database_details(&self) -> CommandResult<DatabaseDetailsResponse> {
        Ok(DatabaseDetailsResponse {
            roots: self
                .roots
                .iter()
                .zip(self.contents.iter())
                .map(|(root, contents)| {
                    #[derive(Default)]
                    struct CountAndSize {
                        count: u64,
                        bytes: u64,
                    }
                    #[derive(Default)]
                    struct FileStats<'a> {
                        total: CountAndSize,
                        by_extension: HashMap<Option<&'a OsStr>, CountAndSize>,
                    }
                    #[derive(Default)]
                    struct GlobalStats<'a> {
                        searchable_files: FileStats<'a>,
                        binary_files: FileStats<'a>,
                    }
                    let stats = root
                        .searchable_files() // TODO: rename
                        .iter()
                        .filter_map(|p| {
                            contents.get(p).map(|c| {
                                let extension = p.extension();
                                match c {
                                    FileContents::Binary(size) => (extension, *size, false),
                                    FileContents::Ascii(vec)
                                    | FileContents::Utf8(vec)
                                    | FileContents::Utf16(vec) => {
                                        (extension, vec.len() as u64, true)
                                    }
                                }
                            })
                        })
                        .fold(
                            GlobalStats::default(),
                            |mut stats, (ext, size, searchable)| {
                                let file_stats = if searchable {
                                    &mut stats.searchable_files
                                } else {
                                    &mut stats.binary_files
                                };
                                file_stats.total.count += 1;
                                file_stats.total.bytes += size;
                                let ext = file_stats.by_extension.entry(ext).or_default();
                                ext.count += 1;
                                ext.bytes += size;
                                stats
                            },
                        );
                    let mut searchable_files_by_extension: Vec<_> = stats
                        .searchable_files
                        .by_extension
                        .iter()
                        .map(|(k, v)| FilesByExtensionDetails {
                            extension: k
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default(),
                            count: v.count,
                            bytes: v.bytes,
                        })
                        .collect();
                    searchable_files_by_extension.sort_by(|a, b| b.bytes.cmp(&a.bytes));
                    let mut binary_files_by_extension: Vec<_> = stats
                        .binary_files
                        .by_extension
                        .iter()
                        .map(|(k, v)| FilesByExtensionDetails {
                            extension: k
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default(),
                            count: v.count,
                            bytes: v.bytes,
                        })
                        .collect();
                    binary_files_by_extension.sort_by(|a, b| b.bytes.cmp(&a.bytes));
                    let (mut large_searchable_files, mut large_binary_files) = root
                        .searchable_files()
                        .iter()
                        .filter_map(|p| {
                            let contents = contents.get(p)?;
                            let (binary, size) = match contents {
                                FileContents::Binary(size) => (true, *size),
                                FileContents::Ascii(vec)
                                | FileContents::Utf16(vec)
                                | FileContents::Utf8(vec) => (false, vec.len() as u64),
                            };
                            if size >= self.configuration.large_file_threshold {
                                Some((
                                    binary,
                                    LargeFileDetails {
                                        path: p.to_string_lossy().to_string(),
                                        bytes: size,
                                    },
                                ))
                            } else {
                                None
                            }
                        })
                        .fold((Vec::new(), Vec::new()), |mut acc, item| {
                            let v = if item.0 { &mut acc.1 } else { &mut acc.0 };
                            v.push(item.1);
                            acc
                        });
                    large_binary_files.sort_by(|a, b| b.bytes.cmp(&a.bytes));
                    large_searchable_files.sort_by(|a, b| b.bytes.cmp(&a.bytes));
                    DatabaseDetailsRoot {
                        root_path: root.directory().path().to_string_lossy().to_string(),
                        num_files_scanned: root.all_files().len() as u64,
                        num_directories_scanned: root.directory().total_child_directories() as u64,
                        num_searchable_files: root.searchable_files().len() as u64,
                        searchable_files_bytes: root
                            .searchable_files()
                            .iter()
                            .filter_map(|f| contents.get(f).map(|c| c.file_size()))
                            .sum(),
                        num_binary_files: stats.binary_files.total.count,
                        binary_files_bytes: stats.binary_files.total.bytes,
                        searchable_files_by_extension,
                        binary_files_by_extension,
                        large_searchable_files,
                        large_binary_files,
                    }
                })
                .collect(),
        })
    }

    // TODO: Don't block service thread while doing this 
    pub async fn register_folder(
        &mut self,
        tx: mpsc::Sender<IndexUpdate>, // TODO: replace with broadcast or watch so no blocking?
        params: FolderRegisterRequest,
    ) -> Result<(), tonic::Status> {
        let span = info_span!("RegisterFolder");
        let _ = span.enter();

        self.send_status(crate::gen::IndexState::Indexing);

        let handle = tokio::runtime::Handle::current();
        tx.send(IndexUpdate::scan_start()).await.ok();
        // TODO: Handle error
        event!(Level::DEBUG, ?params.ignore_file_globs, ?params.path, "Constructing path filter");
        let scan_filter =
            PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_file_globs);
        event!(Level::INFO, ignore_search_globs = ?params.ignore_search_globs, "Constructing search filter");
        let search_filter =
            PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_search_globs);
        // TODO: Dupe detection, check for different ignore globs
        let new_root =
            crate::fs_state::scan_filesystem(Path::new(&params.path), &scan_filter, &search_filter);
        tx.send(IndexUpdate::scan_end()).await.ok();

        // Load the contents of all discovered files in the new root into memory
        let contents: Vec<_> = {
            let tx = tx.clone();
            let searchable_files = new_root.searchable_files();
            event!(Level::INFO, count = ?searchable_files.len(), "Loading contents of indexed files");
            let (contents_tx, mut contents_rx) = mpsc::channel::<FileLoadEvent>(8);
            let total = searchable_files.len();
            let task = handle.spawn(async move {
                let mut loaded = 0;
                while let Some(e) = contents_rx.recv().await {
                    loaded += e.count;
                    if loaded % 100 == 0 {
                        event!(Level::DEBUG, ?loaded, ?total, "Sending files loaded update");
                        tx.send(IndexUpdate::files_loaded(loaded, total, e.path.as_path()))
                            .await.ok();
                    }
                }
            });
            let (_, contents) = tokio::join!(
                task,
                load_files(
                    searchable_files.to_vec(),
                    contents_tx,
                    self.configuration.concurrent_file_reads as usize,
                    self.configuration.max_file_size
                )
            );
            contents
        };

        let contents: HashMap<PathBuf, _> = new_root
            .searchable_files()
            .iter()
            .map(|p| p.to_path_buf())
            .zip(contents)
            .filter_map(|(p, r)| r.ok().map(|c| (p, c)))
            .collect();
        tx.send(IndexUpdate::loaded()).await.ok();
        event!(Level::INFO, "Finished loading file contents");
        // Add the root to make it available for searches
        self.roots.push(new_root);
        self.contents.push(contents);
        // TODO
        self.send_status(crate::gen::IndexState::Ready);
        Ok(())
    }

    pub fn unregister_folder(&mut self, _params: FolderUnregisterRequest) -> CommandResult<()> {
        todo!("");
    }
}
