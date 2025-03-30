/// 产品信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::common::Money>,
    #[prost(enumeration = "Category", tag = "5")]
    pub category: i32,
    #[prost(int32, tag = "6")]
    pub stock_quantity: i32,
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<super::common::Timestamp>,
    #[prost(string, repeated, tag = "8")]
    pub image_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 创建产品请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<super::common::Money>,
    #[prost(enumeration = "Category", tag = "4")]
    pub category: i32,
    #[prost(int32, tag = "5")]
    pub stock_quantity: i32,
    #[prost(string, repeated, tag = "6")]
    pub image_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 创建产品响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::Status>,
    #[prost(message, optional, tag = "2")]
    pub product: ::core::option::Option<Product>,
}
/// 获取产品请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {
    #[prost(string, tag = "1")]
    pub product_id: ::prost::alloc::string::String,
}
/// 获取产品响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::Status>,
    #[prost(message, optional, tag = "2")]
    pub product: ::core::option::Option<Product>,
}
/// 列出产品请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::common::PaginationRequest>,
    #[prost(enumeration = "Category", tag = "2")]
    pub category: i32,
}
/// 列出产品响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::Status>,
    #[prost(message, repeated, tag = "2")]
    pub products: ::prost::alloc::vec::Vec<Product>,
    #[prost(int32, tag = "3")]
    pub total_count: i32,
}
/// 产品类别
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Category {
    Unspecified = 0,
    Electronics = 1,
    Clothing = 2,
    Books = 3,
    Home = 4,
    Sports = 5,
}
impl Category {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Category::Unspecified => "CATEGORY_UNSPECIFIED",
            Category::Electronics => "CATEGORY_ELECTRONICS",
            Category::Clothing => "CATEGORY_CLOTHING",
            Category::Books => "CATEGORY_BOOKS",
            Category::Home => "CATEGORY_HOME",
            Category::Sports => "CATEGORY_SPORTS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "CATEGORY_ELECTRONICS" => Some(Self::Electronics),
            "CATEGORY_CLOTHING" => Some(Self::Clothing),
            "CATEGORY_BOOKS" => Some(Self::Books),
            "CATEGORY_HOME" => Some(Self::Home),
            "CATEGORY_SPORTS" => Some(Self::Sports),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod product_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 产品服务
    #[derive(Debug, Clone)]
    pub struct ProductServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProductServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProductServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProductServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ProductServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// 创建产品
        pub async fn create_product(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateProductResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/product.ProductService/CreateProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("product.ProductService", "CreateProduct"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取产品
        pub async fn get_product(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProductResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/product.ProductService/GetProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("product.ProductService", "GetProduct"));
            self.inner.unary(req, path, codec).await
        }
        /// 列出产品
        pub async fn list_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProductsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/product.ProductService/ListProducts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("product.ProductService", "ListProducts"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod product_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProductServiceServer.
    #[async_trait]
    pub trait ProductService: Send + Sync + 'static {
        /// 创建产品
        async fn create_product(
            &self,
            request: tonic::Request<super::CreateProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateProductResponse>,
            tonic::Status,
        >;
        /// 获取产品
        async fn get_product(
            &self,
            request: tonic::Request<super::GetProductRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProductResponse>,
            tonic::Status,
        >;
        /// 列出产品
        async fn list_products(
            &self,
            request: tonic::Request<super::ListProductsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProductsResponse>,
            tonic::Status,
        >;
    }
    /// 产品服务
    #[derive(Debug)]
    pub struct ProductServiceServer<T: ProductService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ProductService> ProductServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProductServiceServer<T>
    where
        T: ProductService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/product.ProductService/CreateProduct" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::CreateProductRequest>
                    for CreateProductSvc<T> {
                        type Response = super::CreateProductResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_product(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/product.ProductService/GetProduct" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::GetProductRequest>
                    for GetProductSvc<T> {
                        type Response = super::GetProductResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProductRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_product(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProductSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/product.ProductService/ListProducts" => {
                    #[allow(non_camel_case_types)]
                    struct ListProductsSvc<T: ProductService>(pub Arc<T>);
                    impl<
                        T: ProductService,
                    > tonic::server::UnaryService<super::ListProductsRequest>
                    for ListProductsSvc<T> {
                        type Response = super::ListProductsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProductsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_products(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListProductsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ProductService> Clone for ProductServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ProductService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProductService> tonic::server::NamedService for ProductServiceServer<T> {
        const NAME: &'static str = "product.ProductService";
    }
}
