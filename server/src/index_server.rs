use crate::file_contents::load_files;
use crate::file_contents::FileContents;
use crate::file_contents::FileLoadEvent;
use crate::fs_filter::*;
use crate::fs_state::*;
use crate::gen::*;
use crate::search_engine::{get_file_extracts, search_files_contents};

use memory_stats::memory_stats;
use rayon::prelude::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fmt::Debug, time::SystemTime};
use thiserror::Error;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use tonic::Status;
use tracing::instrument;
use tracing::{event, info_span, Level};

#[derive(Error, Debug, Clone)]
pub enum CommandError {
    #[error("Invalid argument {0}")]
    InvalidArgument(String),
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl<T> From<mpsc::error::SendError<T>> for CommandError {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        CommandError::InternalError(value.to_string())
    }
}

impl From<oneshot::error::RecvError> for CommandError {
    fn from(value: oneshot::error::RecvError) -> Self {
        CommandError::InternalError(value.to_string())
    }
}

type CommandResult<T> = Result<T, CommandError>;

// Commands for index state to update, used by the implementer of a tonic service to send commands asynchronously to the index state and receive results
// TODO: Consider making this a private/implementation detail and having the service just call async functions?
#[derive(Debug)]
pub enum Command {
    SetConfiguration(ConfigurationRequest),
    RegisterFolder(
        FolderRegisterRequest,
        broadcast::Sender<CommandResult<IndexUpdate>>, // Document why this is broadcast
    ),
    UnregisterFolder(FolderUnregisterRequest),
    FilePathSearch(
        FilePathSearchRequest,
        oneshot::Sender<FilePathSearchResponse>,
    ),
    FileContentsSearch(
        FileContentsSearchRequest,
        oneshot::Sender<CommandResult<FileContentsSearchResponse>>,
    ),
    GetFileExtracts(
        FileExtractsRequest,
        oneshot::Sender<CommandResult<FileExtractsResponse>>,
    ),
    GetStatusStream(oneshot::Sender<broadcast::Receiver<StatusResponse>>),
}
// TODO: If refactoring to this generic style, work out how to do it with borrow - does this need Pin?
// TODO: Consider Result return type?
type DynCommand = Box<dyn FnOnce(&mut IndexServer) -> CommandResult<()> + Send>;

pub struct IndexServer {
    dyncommand_rx: mpsc::Receiver<DynCommand>,
    command_rx: mpsc::Receiver<Command>,
    status_tx: broadcast::Sender<StatusResponse>,
    configuration: Configuration,
    // TODO: move roots into search_engine.rs
    roots: Vec<Root>,
    // TODO: move contents into roots
    // TODO: document which files should be present in contents
    contents: Vec<HashMap<PathBuf, FileContents>>,
}

pub struct IndexInterface {
    dyncommand_tx: mpsc::Sender<DynCommand>,
}

impl IndexServer {
    // TODO: Probably don't return IndexServer, just run immediately
    // Maybe rename to IndexState and IndexInterface or something - but IndexState exists within gen::
    pub fn new(command_rx: mpsc::Receiver<Command>) -> (Self, IndexInterface) {
        let (status_tx, _) = broadcast::channel(8);
        let (dyncommand_tx, dyncommand_rx) = mpsc::channel(8);
        (
            IndexServer {
                dyncommand_rx,
                command_rx,
                status_tx,
                configuration: Configuration::default(),
                roots: Vec::new(),
                contents: Vec::new(),
            },
            IndexInterface { dyncommand_tx },
        )
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

impl IndexInterface {
    pub async fn get_database_details(&self) -> CommandResult<DatabaseDetailsResponse> {
        let (tx, rx) = oneshot::channel();
        let body = |s: &mut IndexServer| {
            let details = s.get_database_details_internal();
            tx.send(details).map_err(|_| {
                CommandError::InternalError(
                    "Failed to return result on oneshot channel".to_string(),
                )
            })?;
            Ok(())
        };
        self.dyncommand_tx.send(Box::new(body)).await?;
        rx.await?
    }
}

struct Configuration {
    concurrent_file_reads: u32,
    max_file_size: u64,
    large_file_threshold: u64,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            concurrent_file_reads: 64,
            max_file_size: 10 * 1024 * 1024,
            large_file_threshold: 1024 * 1024,
        }
    }
}

impl IndexUpdate {
    fn scan_start() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FilesystemScanStart(
                index_update::FilesystemScanStart::default(),
            )),
        }
    }
    fn scan_end() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FilesystemScanEnd(
                index_update::FilesystemScanEnd::default(),
            )),
        }
    }
    fn loaded() -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FileContentsLoaded(
                index_update::FileContentsLoaded::default(),
            )),
        }
    }
    fn files_loaded(count: usize, total: usize, path: &Path) -> Self {
        let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
        Self {
            timestamp,
            r#type: Some(index_update::Type::FileContentsLoaded(
                index_update::FileContentsLoaded {
                    count: count as u32,
                    total: total as u32,
                    path: path.to_string_lossy().to_string(),
                },
            )),
        }
    }
}

// TODO: Setup directory watcher to feed update commands back into index state
impl IndexServer {
    async fn run_internal(mut self) {
        event!(Level::INFO, "Starting index state command loop");
        loop {
            tokio::select! {
                Some(command) = self.command_rx.recv() => { self.execute_command(command).await.ok(); }
                Some(dyn_command) = self.dyncommand_rx.recv() => { self.execute_dyn_command(dyn_command).await.ok(); }
                else => {
                    break;
                }
            }
        }
        event!(Level::INFO, "Leaving index state command loop");
    }

    // TODO: Return result type and instrument
    #[instrument(err)]
    async fn execute_command(&mut self, c: Command) -> Result<(), tonic::Status> {
        match c {
            Command::SetConfiguration(params) => self.set_configuration(params),
            Command::GetStatusStream(tx) => self.get_status_stream(tx),
            // TODO: Handle overlapping folders
            Command::RegisterFolder(params, tx) => self.register_folder(tx, params).await, // TODO: remove tx from args?
            Command::UnregisterFolder(_params) => {
                // TODO: Remove root directory if it exists
                todo!("");
            }
            Command::FilePathSearch(params, tx) => self.search_file_paths(params, tx), // TODO: remove tx from args?
            Command::FileContentsSearch(params, tx) => {
                let token = CancellationToken::new();
                tx.send(self.search_file_contents(params, token))
                    .map_err(|_| Status::internal(""))
            }
            Command::GetFileExtracts(request, tx) => tx
                .send(self.get_file_extracts(request))
                .map_err(|_| Status::internal("")),
        }
    }

    #[instrument(err, skip_all)]
    async fn execute_dyn_command(&mut self, c: DynCommand) -> Result<(), tonic::Status> {
        Ok(c(self)?)
    }

    fn send_status(&self, state: IndexState) -> Result<(), tonic::Status> {
        let stats = memory_stats().ok_or_else(|| Status::internal(""))?;
        let mem_usage = stats.physical_mem as u64;
        let num_searchable_files = self.get_num_searchable_files();
        self.status_tx
            .send(StatusResponse {
                state: state.into(),
                mem_usage,
                num_searchable_files,
            })
            .map_err(|_| Status::internal(""))?;
        Ok(())
    }

    fn get_num_searchable_files(&self) -> u64 {
        self.roots
            .iter()
            .map(|r| r.searchable_files().len())
            .sum::<usize>() as u64
    }

    fn set_configuration(&mut self, params: ConfigurationRequest) -> Result<(), tonic::Status> {
        if params.concurrent_file_reads != 0 {
            self.configuration.concurrent_file_reads = params.concurrent_file_reads;
        }
        if params.max_file_size != 0 {
            self.configuration.max_file_size = params.max_file_size;
        }
        Ok(())
    }

    fn get_status_stream(
        &self,
        tx: oneshot::Sender<broadcast::Receiver<StatusResponse>>,
    ) -> Result<(), tonic::Status> {
        tx.send(self.status_tx.subscribe()).ok();
        Ok(())
    }

    fn search_file_contents(
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
                    search_files_contents(root.directory().path(), contents, &params, token.clone())
                })
                .collect(),
        })
    }

    fn get_file_extracts(
        &self,
        request: FileExtractsRequest,
    ) -> CommandResult<FileExtractsResponse> {
        let path = PathBuf::from(request.file_path);
        if !path.is_absolute() {
            Err(CommandError::InvalidArgument(
                path.to_string_lossy().to_string(),
            ))
        } else if let Some(contents) = self.contents.iter().find_map(|map| map.get(&path)) {
            let file_extracts =
                get_file_extracts(contents, &request.spans, request.max_extract_length);
            Ok(FileExtractsResponse {
                file_path: path.to_string_lossy().to_string(),
                file_extracts,
            })
        } else {
            Err(CommandError::NotFound(path.to_string_lossy().to_string()))
        }
    }

    fn get_database_details_internal(&self) -> CommandResult<DatabaseDetailsResponse> {
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
                            contents.get(p).and_then(|c| {
                                let extension = p.extension();
                                Some(match c {
                                    FileContents::Binary(size) => (extension, *size, false),
                                    FileContents::Ascii(vec)
                                    | FileContents::Utf8(vec)
                                    | FileContents::Utf16(vec) => {
                                        (extension, vec.len() as u64, true)
                                    }
                                })
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
                        .into_iter()
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

    fn search_file_paths(
        &mut self,
        params: FilePathSearchRequest,
        tx: oneshot::Sender<FilePathSearchResponse>,
    ) -> Result<(), tonic::Status> {
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
        tx.send(FilePathSearchResponse { results, duration })
            .map_err(|_| Status::internal(""))
    }

    async fn register_folder(
        &mut self,
        tx: broadcast::Sender<CommandResult<IndexUpdate>>,
        params: FolderRegisterRequest,
    ) -> Result<(), tonic::Status> {
        let span = info_span!("RegisterFolder");
        let _ = span.enter();

        self.send_status(IndexState::Indexing).ok();

        let handle = tokio::runtime::Handle::current();
        tx.send(Ok(IndexUpdate::scan_start())).ok();
        // TODO: Handle error
        event!(Level::DEBUG, ?params.ignore_file_globs, ?params.path, "Constructing path filter");
        let scan_filter =
            PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_file_globs);
        event!(Level::INFO, ignore_search_globs = ?params.ignore_search_globs, "Constructing search filter");
        let search_filter =
            PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_search_globs);
        // TODO: Dupe detection, check for different ignore globs
        let new_root = scan_filesystem(Path::new(&params.path), &scan_filter, &search_filter);
        tx.send(Ok(IndexUpdate::scan_end())).ok();

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
                        tx.send(Ok(IndexUpdate::files_loaded(
                            loaded,
                            total,
                            e.path.as_path(),
                        )))
                        .ok();
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
        tx.send(Ok(IndexUpdate::loaded())).ok();
        event!(Level::INFO, "Finished loading file contents");
        // Add the root to make it available for searches
        self.roots.push(new_root);
        self.contents.push(contents);
        self.send_status(IndexState::Ready).ok();
        Ok(())
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
