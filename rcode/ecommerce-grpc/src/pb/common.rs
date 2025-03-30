/// 通用响应状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// 分页请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginationRequest {
    #[prost(int32, tag = "1")]
    pub page: i32,
    #[prost(int32, tag = "2")]
    pub page_size: i32,
}
/// 时间戳
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
/// 货币
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    #[prost(string, tag = "1")]
    pub currency_code: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub units: i64,
    #[prost(int32, tag = "3")]
    pub nanos: i32,
}
/// 地址
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, tag = "1")]
    pub street_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub city: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub zip_code: ::prost::alloc::string::String,
}
