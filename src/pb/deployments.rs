// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Deployment {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub total_supply: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub decimals: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub blocknumber: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub timestamp_seconds: i64,
}
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
    #[prost(int64, tag="6")]
    pub timestamp_seconds: i64,
}
// @@protoc_insertion_point(module)
