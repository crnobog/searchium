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
pub struct EmptyRequest {
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
    #[prost(oneof="index_update::Type", tags="10, 11")]
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
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="10")]
        FilesystemScanStart(FilesystemScanStart),
        #[prost(message, tag="11")]
        FilesystemScanEnd(FilesystemScanEnd),
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