/// 订单项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderItem {
    #[prost(string, tag = "1")]
    pub product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub product_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub quantity: i32,
    #[prost(message, optional, tag = "4")]
    pub unit_price: ::core::option::Option<super::common::Money>,
}
/// 订单
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<OrderItem>,
    #[prost(message, optional, tag = "4")]
    pub total_amount: ::core::option::Option<super::common::Money>,
    #[prost(enumeration = "OrderStatus", tag = "5")]
    pub status: i32,
    #[prost(message, optional, tag = "6")]
    pub created_at: ::core::option::Option<super::common::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub updated_at: ::core::option::Option<super::common::Timestamp>,
    #[prost(message, optional, tag = "8")]
    pub shipping_address: ::core::option::Option<super::common::Address>,
}
/// 创建订单请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrderRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<OrderItem>,
    #[prost(message, optional, tag = "3")]
    pub shipping_address: ::core::option::Option<super::common::Address>,
}
/// 创建订单响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::Status>,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<Order>,
}
/// 获取订单请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderRequest {
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
}
/// 获取订单响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::Status>,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<Order>,
}
/// 列出用户订单请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserOrdersRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::common::PaginationRequest>,
}
/// 列出用户订单响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserOrdersResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::Status>,
    #[prost(message, repeated, tag = "2")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(int32, tag = "3")]
    pub total_count: i32,
}
/// 更新订单状态请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrderStatusRequest {
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderStatus", tag = "2")]
    pub status: i32,
}
/// 更新订单状态响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrderStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::Status>,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<Order>,
}
/// 订单状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    Unspecified = 0,
    Created = 1,
    Processing = 2,
    Shipped = 3,
    Delivered = 4,
    Cancelled = 5,
}
impl OrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderStatus::Unspecified => "ORDER_STATUS_UNSPECIFIED",
            OrderStatus::Created => "ORDER_STATUS_CREATED",
            OrderStatus::Processing => "ORDER_STATUS_PROCESSING",
            OrderStatus::Shipped => "ORDER_STATUS_SHIPPED",
            OrderStatus::Delivered => "ORDER_STATUS_DELIVERED",
            OrderStatus::Cancelled => "ORDER_STATUS_CANCELLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_STATUS_CREATED" => Some(Self::Created),
            "ORDER_STATUS_PROCESSING" => Some(Self::Processing),
            "ORDER_STATUS_SHIPPED" => Some(Self::Shipped),
            "ORDER_STATUS_DELIVERED" => Some(Self::Delivered),
            "ORDER_STATUS_CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod order_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 订单服务
    #[derive(Debug, Clone)]
    pub struct OrderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderServiceClient<tonic::transport::Channel> {
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
    impl<T> OrderServiceClient<T>
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
        ) -> OrderServiceClient<InterceptedService<T, F>>
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
            OrderServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// 创建订单
        pub async fn create_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrderResponse>,
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
                "/order.OrderService/CreateOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("order.OrderService", "CreateOrder"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取订单
        pub async fn get_order(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
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
                "/order.OrderService/GetOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("order.OrderService", "GetOrder"));
            self.inner.unary(req, path, codec).await
        }
        /// 列出用户订单
        pub async fn list_user_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserOrdersResponse>,
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
                "/order.OrderService/ListUserOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("order.OrderService", "ListUserOrders"));
            self.inner.unary(req, path, codec).await
        }
        /// 更新订单状态
        pub async fn update_order_status(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrderStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrderStatusResponse>,
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
                "/order.OrderService/UpdateOrderStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("order.OrderService", "UpdateOrderStatus"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod order_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrderServiceServer.
    #[async_trait]
    pub trait OrderService: Send + Sync + 'static {
        /// 创建订单
        async fn create_order(
            &self,
            request: tonic::Request<super::CreateOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrderResponse>,
            tonic::Status,
        >;
        /// 获取订单
        async fn get_order(
            &self,
            request: tonic::Request<super::GetOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
            tonic::Status,
        >;
        /// 列出用户订单
        async fn list_user_orders(
            &self,
            request: tonic::Request<super::ListUserOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserOrdersResponse>,
            tonic::Status,
        >;
        /// 更新订单状态
        async fn update_order_status(
            &self,
            request: tonic::Request<super::UpdateOrderStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrderStatusResponse>,
            tonic::Status,
        >;
    }
    /// 订单服务
    #[derive(Debug)]
    pub struct OrderServiceServer<T: OrderService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OrderService> OrderServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrderServiceServer<T>
    where
        T: OrderService,
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
                "/order.OrderService/CreateOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::CreateOrderRequest>
                    for CreateOrderSvc<T> {
                        type Response = super::CreateOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_order(request).await
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
                        let method = CreateOrderSvc(inner);
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
                "/order.OrderService/GetOrder" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::GetOrderRequest>
                    for GetOrderSvc<T> {
                        type Response = super::GetOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_order(request).await };
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
                        let method = GetOrderSvc(inner);
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
                "/order.OrderService/ListUserOrders" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserOrdersSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::ListUserOrdersRequest>
                    for ListUserOrdersSvc<T> {
                        type Response = super::ListUserOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_user_orders(request).await
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
                        let method = ListUserOrdersSvc(inner);
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
                "/order.OrderService/UpdateOrderStatus" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOrderStatusSvc<T: OrderService>(pub Arc<T>);
                    impl<
                        T: OrderService,
                    > tonic::server::UnaryService<super::UpdateOrderStatusRequest>
                    for UpdateOrderStatusSvc<T> {
                        type Response = super::UpdateOrderStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOrderStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_order_status(request).await
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
                        let method = UpdateOrderStatusSvc(inner);
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
    impl<T: OrderService> Clone for OrderServiceServer<T> {
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
    impl<T: OrderService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OrderService> tonic::server::NamedService for OrderServiceServer<T> {
        const NAME: &'static str = "order.OrderService";
    }
}
