// @generated
// import "google/type/datetime.proto"

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
include!("searchium.v2.serde.rs");
include!("searchium.v2.tonic.rs");
// @@protoc_insertion_point(module)