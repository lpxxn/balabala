/// 通用响应状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// 货币金额
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    #[prost(string, tag = "1")]
    pub currency_code: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub amount: i64,
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
