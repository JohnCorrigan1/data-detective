// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Deployment {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub blocknumber: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub timestamp_seconds: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub code: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="8")]
    pub storage_changes: ::prost::alloc::vec::Vec<Change>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc721Mint {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub blocknumber: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub timestamp_seconds: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Change {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterProto {
    #[prost(message, repeated, tag="1")]
    pub mints: ::prost::alloc::vec::Vec<Erc721Mint>,
    #[prost(message, repeated, tag="2")]
    pub contracts: ::prost::alloc::vec::Vec<Erc721Deployment>,
}
// @@protoc_insertion_point(module)
