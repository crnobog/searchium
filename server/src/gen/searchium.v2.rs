// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigurationRequest {
    /// Max number of files to try and load simultaneously 
    #[prost(uint32, tag="1")]
    pub concurrent_file_reads: u32,
    /// Max size of files to index
    #[prost(uint64, tag="2")]
    pub max_file_size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigurationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericResponse {
    #[prost(enumeration="GenericError", tag="1")]
    pub error: i32,
    #[prost(string, tag="2")]
    pub log_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderRegisterRequest {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub ignore_file_globs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub ignore_search_globs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderUnregisterRequest {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexUpdate {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof="index_update::Type", tags="10, 11, 12")]
    pub r#type: ::core::option::Option<index_update::Type>,
}
/// Nested message and enum types in `IndexUpdate`.
pub mod index_update {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FilesystemScanStart {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FilesystemScanEnd {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FileContentsLoaded {
        #[prost(uint32, tag="1")]
        pub count: u32,
        #[prost(uint32, tag="2")]
        pub total: u32,
        #[prost(string, tag="3")]
        pub path: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="10")]
        FilesystemScanStart(FilesystemScanStart),
        #[prost(message, tag="11")]
        FilesystemScanEnd(FilesystemScanEnd),
        #[prost(message, tag="12")]
        FileContentsLoaded(FileContentsLoaded),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilePathSearchRequest {
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub max_results: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilePathSearchResponse {
    #[prost(string, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileContentsSearchRequest {
    #[prost(string, tag="1")]
    pub query_string: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub file_path_pattern: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub max_results: u32,
    #[prost(bool, tag="4")]
    pub match_case: bool,
    #[prost(bool, tag="5")]
    pub match_whole_word: bool,
    #[prost(bool, tag="6")]
    pub regex: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Span {
    #[prost(uint32, tag="1")]
    pub offset_bytes: u32,
    #[prost(uint32, tag="2")]
    pub length_bytes: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileContentsSearchHit {
    #[prost(string, tag="1")]
    pub file_relative_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub spans: ::prost::alloc::vec::Vec<Span>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileContentsSearchRootResult {
    #[prost(string, tag="1")]
    pub root_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub hits: ::prost::alloc::vec::Vec<FileContentsSearchHit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileContentsSearchResponse {
    #[prost(message, repeated, tag="1")]
    pub roots: ::prost::alloc::vec::Vec<FileContentsSearchRootResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileExtractsRequest {
    #[prost(string, tag="1")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub spans: ::prost::alloc::vec::Vec<Span>,
    #[prost(uint32, tag="3")]
    pub max_extract_length: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileExtract {
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub offset: u32,
    #[prost(uint32, tag="3")]
    pub length: u32,
    #[prost(uint32, tag="4")]
    pub line_number: u32,
    #[prost(uint32, tag="5")]
    pub column_number: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileExtractsResponse {
    #[prost(string, tag="1")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub file_extracts: ::prost::alloc::vec::Vec<FileExtract>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessInfoRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessInfoResponse {
    #[prost(uint64, tag="1")]
    pub physical_memory: u64,
    #[prost(uint64, tag="2")]
    pub virtual_memory: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseDetailsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseDetailsRoot {
    #[prost(string, tag="1")]
    pub root_path: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub num_files_scanned: u64,
    #[prost(uint64, tag="3")]
    pub num_directories_scanned: u64,
    #[prost(uint64, tag="4")]
    pub num_searchable_files: u64,
    #[prost(uint64, tag="5")]
    pub searchable_files_bytes: u64,
    #[prost(uint64, tag="6")]
    pub num_binary_files: u64,
    #[prost(uint64, tag="7")]
    pub binary_files_bytes: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseDetailsResponse {
    #[prost(message, repeated, tag="1")]
    pub roots: ::prost::alloc::vec::Vec<DatabaseDetailsRoot>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GenericError {
    None = 0,
}
impl GenericError {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GenericError::None => "GENERIC_ERROR_NONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GENERIC_ERROR_NONE" => Some(Self::None),
            _ => None,
        }
    }
}
include!("searchium.v2.tonic.rs");
// @@protoc_insertion_point(module)