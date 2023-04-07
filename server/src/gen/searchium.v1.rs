// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpcMessage {
    #[prost(int64, tag="1")]
    pub request_id: i64,
    #[prost(string, tag="2")]
    pub protocol: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<IpcMessageData>,
    #[prost(oneof="ipc_message::RequestResponse", tags="10, 11")]
    pub request_response: ::core::option::Option<ipc_message::RequestResponse>,
}
/// Nested message and enum types in `IpcMessage`.
pub mod ipc_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestResponse {
        #[prost(message, tag="10")]
        Request(super::IpcRequest),
        #[prost(message, tag="11")]
        Response(super::IpcResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpcMessageData {
    #[prost(oneof="ipc_message_data::Subtype", tags="10, 11, 12")]
    pub subtype: ::core::option::Option<ipc_message_data::Subtype>,
}
/// Nested message and enum types in `IpcMessageData`.
pub mod ipc_message_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subtype {
        #[prost(message, tag="10")]
        IpcStringData(super::IpcStringData),
        #[prost(message, tag="11")]
        TypedMessage(super::TypedMessage),
        #[prost(message, tag="12")]
        ErrorResponse(super::ErrorResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpcStringData {
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedMessage {
    #[prost(string, tag="1")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(oneof="typed_message::Subtype", tags="10, 11, 12")]
    pub subtype: ::core::option::Option<typed_message::Subtype>,
}
/// Nested message and enum types in `TypedMessage`.
pub mod typed_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subtype {
        #[prost(message, tag="10")]
        TypedRequest(super::TypedRequest),
        #[prost(message, tag="11")]
        TypedResponse(super::TypedResponse),
        #[prost(message, tag="12")]
        TypedEvent(super::TypedEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub full_type_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub stack_trace: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="4")]
    pub inner_error: ::core::option::Option<::prost::alloc::boxed::Box<ErrorResponse>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedRequest {
    #[prost(oneof="typed_request::Subtype", tags="10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28")]
    pub subtype: ::core::option::Option<typed_request::Subtype>,
}
/// Nested message and enum types in `TypedRequest`.
pub mod typed_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subtype {
        #[prost(message, tag="10")]
        RegisterFileRequest(super::RegisterFileRequest),
        #[prost(message, tag="11")]
        UnregisterFileRequest(super::UnregisterFileRequest),
        #[prost(message, tag="12")]
        GetFileSystemRequest(super::GetFileSystemRequest),
        #[prost(message, tag="13")]
        SearchFilePathsRequest(super::SearchFilePathsRequest),
        #[prost(message, tag="14")]
        SearchCodeRequest(super::SearchCodeRequest),
        #[prost(message, tag="15")]
        GetFileSystemVersionRequest(super::GetFileSystemVersionRequest),
        #[prost(message, tag="16")]
        GetFileExtractsRequest(super::GetFileExtractsRequest),
        #[prost(message, tag="18")]
        GetDirectoryStatisticsRequest(super::GetDirectoryStatisticsRequest),
        #[prost(message, tag="19")]
        RefreshFilesystemTreeRequest(super::RefreshFileSystemTreeRequest),
        #[prost(message, tag="20")]
        GetDatabaseStatisticsRequest(super::GetDatabaseStatisticsRequest),
        #[prost(message, tag="21")]
        PauseIndexingRequest(super::PauseIndexingRequest),
        #[prost(message, tag="22")]
        ResumeIndexingRequest(super::ResumeIndexingRequest),
        #[prost(message, tag="23")]
        GetDatabaseDetailsRequest(super::GetDatabaseDetailsRequest),
        #[prost(message, tag="24")]
        GetProjectDetailsRequest(super::GetProjectDetailsRequest),
        #[prost(message, tag="25")]
        GetDirectoryDetailsRequest(super::GetDirectoryDetailsRequest),
        #[prost(message, tag="26")]
        GetFilesystemTreeRequest(super::GetFilesystemTreeRequest),
        #[prost(message, tag="27")]
        GetDirectoryEntriesRequest(super::GetDirectoryEntriesRequest),
        #[prost(message, tag="28")]
        GetDirectoryEntriesMultipleRequest(super::GetDirectoryEntriesMultipleRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedResponse {
    #[prost(oneof="typed_response::Subtype", tags="10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26")]
    pub subtype: ::core::option::Option<typed_response::Subtype>,
}
/// Nested message and enum types in `TypedResponse`.
pub mod typed_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subtype {
        #[prost(message, tag="10")]
        DoneResponse(super::DoneResponse),
        #[prost(message, tag="11")]
        GetFileSystemResponse(super::GetFileSystemResponse),
        #[prost(message, tag="12")]
        SearchFilePathsResponse(super::SearchFilePathsResponse),
        #[prost(message, tag="13")]
        SearchCodeResponse(super::SearchCodeResponse),
        #[prost(message, tag="14")]
        GetFileSystemVersionResponse(super::GetFileSystemVersionResponse),
        #[prost(message, tag="16")]
        GetFileExtractsResponse(super::GetFileExtractsResponse),
        #[prost(message, tag="17")]
        GetDirectoryStatisticsResponse(super::GetDirectoryStatisticsResponse),
        #[prost(message, tag="18")]
        RefreshFileSystemTreeResponse(super::RefreshFileSystemTreeResponse),
        #[prost(message, tag="19")]
        GetDatabaseStatisticsResponse(super::GetDatabaseStatisticsResponse),
        #[prost(message, tag="20")]
        PauseResumeIndexingResponse(super::PauseResumeIndexingResponse),
        #[prost(message, tag="21")]
        GetDatabaseDetailsResponse(super::GetDatabaseDetailsResponse),
        #[prost(message, tag="22")]
        GetProjectDetailsResponse(super::GetProjectDetailsResponse),
        #[prost(message, tag="23")]
        GetDirectoryDetailsResponse(super::GetDirectoryDetailsResponse),
        #[prost(message, tag="24")]
        GetFilesystemTreeResponse(super::GetFileSystemTreeResponse),
        #[prost(message, tag="25")]
        GetDirectoryEntriesResponse(super::GetDirectoryEntriesResponse),
        #[prost(message, tag="26")]
        GetDirectoryEntriesMultipleResponse(super::GetDirectoryEntriesMultipleResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedEvent {
    #[prost(oneof="typed_event::Subtype", tags="10, 11, 21")]
    pub subtype: ::core::option::Option<typed_event::Subtype>,
}
/// Nested message and enum types in `TypedEvent`.
pub mod typed_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subtype {
        #[prost(message, tag="10")]
        PairedTypedEvent(super::PairedTypedEvent),
        #[prost(message, tag="11")]
        ProgressReportEvent(super::ProgressReportEvent),
        #[prost(message, tag="21")]
        IndexingServerStateChangedEvent(super::IndexingServerStateChangedEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairedTypedEvent {
    #[prost(int64, tag="1")]
    pub operation_id: i64,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ErrorResponse>,
    #[prost(oneof="paired_typed_event::Subtype", tags="10, 11, 12, 13, 14")]
    pub subtype: ::core::option::Option<paired_typed_event::Subtype>,
}
/// Nested message and enum types in `PairedTypedEvent`.
pub mod paired_typed_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subtype {
        #[prost(message, tag="10")]
        FileSystemScanStarted(super::FileSystemScanStarted),
        #[prost(message, tag="11")]
        FileSystemScanFinished(super::FileSystemScanFinished),
        #[prost(message, tag="12")]
        SearchEngineFilesLoading(super::SearchEngineFilesLoading),
        #[prost(message, tag="13")]
        SearchEngineFilesLoadingProgress(super::SearchEngineFilesLoadingProgress),
        #[prost(message, tag="14")]
        SearchEngineFilesLoaded(super::SearchEngineFilesLoaded),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemScanStarted {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemScanFinished {
    #[prost(int32, tag="1")]
    pub old_version: i32,
    #[prost(int32, tag="2")]
    pub new_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEngineFilesLoading {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEngineFilesLoadingProgress {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEngineFilesLoaded {
    #[prost(int64, tag="1")]
    pub tree_version: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressReportEvent {
    #[prost(string, tag="1")]
    pub display_text: ::prost::alloc::string::String,
    /// number of completed steps <= total
    #[prost(int32, tag="2")]
    pub completed: i32,
    /// total expected number of steps 
    #[prost(int32, tag="3")]
    pub total: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexingServerStateChangedEvent {
    #[prost(enumeration="IndexingServerStatus", tag="1")]
    pub server_status: i32,
    #[prost(message, optional, tag="2")]
    pub last_index_updated_utc: ::core::option::Option<super::super::bcl::DateTime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpcRequest {
    #[prost(bool, tag="3")]
    pub run_on_sequential_queue: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpcResponse {
    #[prost(message, optional, tag="10")]
    pub ipc_event: ::core::option::Option<IpcEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpcEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterFileRequest {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterFileRequest {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSystemRequest {
    #[prost(int32, tag="1")]
    pub known_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFilePathsRequest {
    #[prost(message, optional, tag="1")]
    pub search_params: ::core::option::Option<SearchParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCodeRequest {
    #[prost(message, optional, tag="1")]
    pub search_params: ::core::option::Option<SearchParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSystemVersionRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileExtractsRequest {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub positions: ::prost::alloc::vec::Vec<FilePositionSpan>,
    #[prost(int32, tag="3")]
    pub max_extract_length: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryStatisticsRequest {
    #[prost(string, tag="1")]
    pub directory_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshFileSystemTreeRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseStatisticsRequest {
    #[prost(bool, tag="1")]
    pub force_garbage_collection: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseIndexingRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeIndexingRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseDetailsRequest {
    #[prost(int32, tag="1")]
    pub max_files_by_extension_details_count: i32,
    #[prost(int32, tag="2")]
    pub max_large_files_details_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectDetailsRequest {
    #[prost(string, tag="1")]
    pub project_path: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub max_files_by_extension_details_count: i32,
    #[prost(int32, tag="3")]
    pub max_large_files_details_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryDetailsRequest {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub max_files_by_extension_details_count: i32,
    #[prost(int32, tag="3")]
    pub max_large_files_details_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFilesystemTreeRequest {
    #[prost(int32, tag="1")]
    pub known_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryEntriesRequest {
    #[prost(string, tag="1")]
    pub project_path: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub directory_relative_path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryEntriesMultipleRequest {
    #[prost(string, tag="1")]
    pub project_path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub relative_path_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilePositionSpan {
    #[prost(int32, tag="1")]
    pub position: i32,
    #[prost(int32, tag="2")]
    pub length: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchParams {
    #[prost(string, tag="1")]
    pub search_string: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub file_path_pattern: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub max_results: i32,
    #[prost(bool, tag="4")]
    pub match_case: bool,
    #[prost(bool, tag="5")]
    pub match_whole_word: bool,
    #[prost(bool, tag="6")]
    pub include_sym_links: bool,
    #[prost(bool, tag="7")]
    pub regex: bool,
    #[prost(bool, tag="8")]
    pub use_re2_engine: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoneResponse {
    #[prost(string, tag="1")]
    pub info: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSystemResponse {
    #[prost(message, optional, tag="1")]
    pub file_system_tree_obsolete: ::core::option::Option<FileSystemTreeObsolete>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFilePathsResponse {
    #[prost(message, optional, tag="1")]
    pub search_result: ::core::option::Option<FileSystemEntry>,
    #[prost(int64, tag="2")]
    pub hit_count: i64,
    #[prost(int64, tag="3")]
    pub total_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCodeResponse {
    #[prost(message, optional, tag="1")]
    pub search_results: ::core::option::Option<FileSystemEntry>,
    #[prost(int64, tag="2")]
    pub hit_count: i64,
    #[prost(int64, tag="3")]
    pub searched_file_count: i64,
    #[prost(int64, tag="4")]
    pub total_file_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSystemVersionResponse {
    #[prost(int32, tag="1")]
    pub version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileExtractsResponse {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub file_extracts: ::prost::alloc::vec::Vec<FileExtract>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryStatisticsResponse {
    #[prost(int32, tag="1")]
    pub directory_count: i32,
    #[prost(int32, tag="2")]
    pub file_count: i32,
    #[prost(int32, tag="3")]
    pub indexed_file_count: i32,
    #[prost(int64, tag="4")]
    pub total_indexed_file_size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshFileSystemTreeResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseStatisticsResponse {
    #[prost(int32, tag="1")]
    pub project_count: i32,
    #[prost(int64, tag="2")]
    pub file_count: i64,
    #[prost(int64, tag="3")]
    pub searchable_file_count: i64,
    #[prost(int64, tag="4")]
    pub server_native_memory_usage: i64,
    #[prost(int64, tag="5")]
    pub server_gc_memory_usage: i64,
    #[prost(message, optional, tag="6")]
    pub index_last_updated_utc: ::core::option::Option<super::super::bcl::DateTime>,
    #[prost(enumeration="IndexingServerStatus", tag="7")]
    pub server_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseResumeIndexingResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseDetailsResponse {
    #[prost(message, repeated, tag="1")]
    pub project_details: ::prost::alloc::vec::Vec<ProjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectDetailsResponse {
    #[prost(message, optional, tag="1")]
    pub project_details: ::core::option::Option<ProjectDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryDetailsResponse {
    #[prost(message, optional, tag="1")]
    pub directory_details: ::core::option::Option<DirectoryDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSystemTreeResponse {
    #[prost(message, optional, tag="1")]
    pub file_system_tree: ::core::option::Option<FileSystemTree>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryEntriesResponse {
    #[prost(message, optional, tag="1")]
    pub directory_entry: ::core::option::Option<DirectoryEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryEntriesMultipleResponse {
    #[prost(message, repeated, tag="1")]
    pub directory_entries: ::prost::alloc::vec::Vec<OptionalDirectoryEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemTree {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(message, repeated, tag="2")]
    pub projects: ::prost::alloc::vec::Vec<ProjectEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectEntry {
    #[prost(string, tag="1")]
    pub root_path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemTreeObsolete {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(message, optional, tag="2")]
    pub root: ::core::option::Option<DirectoryEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectDetails {
    #[prost(string, tag="1")]
    pub root_path: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub directory_details: ::core::option::Option<DirectoryDetails>,
    #[prost(message, optional, tag="3")]
    pub project_configuration_details: ::core::option::Option<ProjectConfigurationDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectConfigurationDetails {
    #[prost(message, optional, tag="1")]
    pub ignore_paths_section: ::core::option::Option<ProjectConfigurationSectionDetails>,
    #[prost(message, optional, tag="2")]
    pub ignore_searchable_files_section: ::core::option::Option<ProjectConfigurationSectionDetails>,
    #[prost(message, optional, tag="3")]
    pub include_searchable_files_section: ::core::option::Option<ProjectConfigurationSectionDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectConfigurationSectionDetails {
    #[prost(string, tag="1")]
    pub containing_file_path: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub contents: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalDirectoryEntry {
    #[prost(bool, tag="1")]
    pub has_value: bool,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<DirectoryEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryEntry {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<FileSystemEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileEntry {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemEntry {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<FileSystemEntryData>,
    #[prost(oneof="file_system_entry::Subtype", tags="10, 11")]
    pub subtype: ::core::option::Option<file_system_entry::Subtype>,
}
/// Nested message and enum types in `FileSystemEntry`.
pub mod file_system_entry {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subtype {
        #[prost(message, tag="10")]
        FileEntry(super::FileEntry),
        #[prost(message, tag="11")]
        DirectoryEntry(super::DirectoryEntry),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemEntryData {
    #[prost(message, optional, tag="10")]
    pub file_positions_data: ::core::option::Option<FilePositionsData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilePositionsData {
    #[prost(message, repeated, tag="1")]
    pub positions: ::prost::alloc::vec::Vec<FilePositionSpan>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryDetails {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub directory_count: i64,
    #[prost(int64, tag="3")]
    pub file_count: i64,
    #[prost(int64, tag="4")]
    pub searchable_files_count: i64,
    #[prost(int64, tag="5")]
    pub searchable_files_byte_length: i64,
    #[prost(int64, tag="6")]
    pub binary_files_count: i64,
    #[prost(int64, tag="7")]
    pub binary_files_byte_length: i64,
    #[prost(message, repeated, tag="8")]
    pub searchable_files_by_extension_details: ::prost::alloc::vec::Vec<FileByExtensionDetails>,
    #[prost(message, repeated, tag="9")]
    pub large_searchable_file_details: ::prost::alloc::vec::Vec<LargeFileDetails>,
    #[prost(message, repeated, tag="10")]
    pub binary_files_by_extension_details: ::prost::alloc::vec::Vec<FileByExtensionDetails>,
    #[prost(message, repeated, tag="11")]
    pub large_binary_files_details: ::prost::alloc::vec::Vec<LargeFileDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileByExtensionDetails {
    #[prost(string, tag="1")]
    pub file_extension: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub file_count: i64,
    #[prost(int64, tag="3")]
    pub file_byte_length: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeFileDetails {
    #[prost(string, tag="1")]
    pub relative_path: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub byte_length: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileExtract {
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub offset: i32,
    #[prost(int32, tag="3")]
    pub length: i32,
    #[prost(int32, tag="4")]
    pub line_number: i32,
    #[prost(int32, tag="5")]
    pub column_number: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndexingServerStatus {
    Idle = 0,
    Paused = 1,
    Yield = 2,
    Busy = 3,
}
impl IndexingServerStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IndexingServerStatus::Idle => "INDEXING_SERVER_STATUS_IDLE",
            IndexingServerStatus::Paused => "INDEXING_SERVER_STATUS_PAUSED",
            IndexingServerStatus::Yield => "INDEXING_SERVER_STATUS_YIELD",
            IndexingServerStatus::Busy => "INDEXING_SERVER_STATUS_BUSY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INDEXING_SERVER_STATUS_IDLE" => Some(Self::Idle),
            "INDEXING_SERVER_STATUS_PAUSED" => Some(Self::Paused),
            "INDEXING_SERVER_STATUS_YIELD" => Some(Self::Yield),
            "INDEXING_SERVER_STATUS_BUSY" => Some(Self::Busy),
            _ => None,
        }
    }
}
include!("searchium.v1.serde.rs");
// @@protoc_insertion_point(module)