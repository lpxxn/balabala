use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};
use tonic::{Request, Response, Status, transport::Server};
use uuid::Uuid;

// 导入生成的protobuf模块
mod pb;
use pb::common::*;
use pb::order::*;
use pb::product::*;
use pb::user::*;

// 将 protobuf 中的 Status 重命名为 ApiStatus，避免与 tonic 的 Status 冲突
use pb::common::Status as ApiStatus;

// 用户服务实现
struct UserServiceImpl {
    users: Arc<Mutex<HashMap<String, pb::user::User>>>,
}

#[tonic::async_trait]
impl user_service_server::UserService for UserServiceImpl {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let user_id = Uuid::new_v4().to_string();

        let now = chrono::Utc::now();
        let timestamp = Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        };

        let user = User {
            id: user_id.clone(),
            username: req.username,
            email: req.email,
            full_name: req.full_name,
            created_at: Some(timestamp),
            address: req.address,
        };

        self.users.lock().unwrap().insert(user_id, user.clone());

        Ok(Response::new(CreateUserResponse {
            status: Some(ApiStatus {
                code: 0,
                message: "User created successfully".to_string(),
            }),
            user: Some(user),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        let users = self.users.lock().unwrap();

        match users.get(&req.user_id) {
            Some(user) => Ok(Response::new(GetUserResponse {
                status: Some(ApiStatus {
                    code: 0,
                    message: "User found".to_string(),
                }),
                user: Some(user.clone()),
            })),
            None => Err(Status::not_found("User not found")),
        }
    }

    async fn list_users(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        let req = request.into_inner();
        let pagination = req.pagination.unwrap_or(PaginationRequest {
            page: 1,
            page_size: 10,
        });

        let users = self.users.lock().unwrap();
        let all_users: Vec<User> = users.values().cloned().collect();
        let total_count = all_users.len() as i32;

        // 简单分页
        let start = ((pagination.page - 1) * pagination.page_size) as usize;
        let end = std::cmp::min(start + pagination.page_size as usize, all_users.len());
        let paged_users = if start < all_users.len() {
            all_users[start..end].to_vec()
        } else {
            vec![]
        };

        Ok(Response::new(ListUsersResponse {
            status: Some(ApiStatus {
                code: 0,
                message: "Users retrieved successfully".to_string(),
            }),
            users: paged_users,
            total_count,
        }))
    }
}

// 产品服务实现
struct ProductServiceImpl {
    products: Arc<Mutex<HashMap<String, Product>>>,
}

#[tonic::async_trait]
impl product_service_server::ProductService for ProductServiceImpl {
    async fn create_product(
        &self,
        request: Request<CreateProductRequest>,
    ) -> Result<Response<CreateProductResponse>, Status> {
        let req = request.into_inner();
        let product_id = Uuid::new_v4().to_string();

        let now = chrono::Utc::now();
        let timestamp = Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        };

        let product = Product {
            id: product_id.clone(),
            name: req.name,
            description: req.description,
            price: req.price,
            category: req.category,
            stock_quantity: req.stock_quantity,
            created_at: Some(timestamp),
            image_urls: req.image_urls,
        };

        self.products
            .lock()
            .unwrap()
            .insert(product_id, product.clone());

        Ok(Response::new(CreateProductResponse {
            status: Some(ApiStatus {
                code: 0,
                message: "Product created successfully".to_string(),
            }),
            product: Some(product),
        }))
    }

    async fn get_product(
        &self,
        request: Request<GetProductRequest>,
    ) -> Result<Response<GetProductResponse>, Status> {
        let req = request.into_inner();
        let products = self.products.lock().unwrap();

        match products.get(&req.product_id) {
            Some(product) => Ok(Response::new(GetProductResponse {
                status: Some(ApiStatus {
                    code: 0,
                    message: "Product found".to_string(),
                }),
                product: Some(product.clone()),
            })),
            None => Err(Status::not_found("Product not found")),
        }
    }

    async fn list_products(
        &self,
        request: Request<ListProductsRequest>,
    ) -> Result<Response<ListProductsResponse>, Status> {
        let req = request.into_inner();
        let pagination = req.pagination.unwrap_or(PaginationRequest {
            page: 1,
            page_size: 10,
        });

        let products = self.products.lock().unwrap();
        let mut all_products: Vec<Product> = products.values().cloned().collect();

        // 如果指定了类别，进行过滤
        if req.category != 0 {
            all_products.retain(|p| p.category == req.category);
        }

        let total_count = all_products.len() as i32;

        // 简单分页
        let start = ((pagination.page - 1) * pagination.page_size) as usize;
        let end = std::cmp::min(start + pagination.page_size as usize, all_products.len());
        let paged_products = if start < all_products.len() {
            all_products[start..end].to_vec()
        } else {
            vec![]
        };

        Ok(Response::new(ListProductsResponse {
            status: Some(ApiStatus {
                code: 0,
                message: "Products retrieved successfully".to_string(),
            }),
            products: paged_products,
            total_count,
        }))
    }
}

// 订单服务实现
struct OrderServiceImpl {
    orders: Arc<Mutex<HashMap<String, Order>>>,
    products: Arc<Mutex<HashMap<String, Product>>>,
}

#[tonic::async_trait]
impl order_service_server::OrderService for OrderServiceImpl {
    async fn create_order(
        &self,
        request: Request<CreateOrderRequest>,
    ) -> Result<Response<CreateOrderResponse>, Status> {
        let req = request.into_inner();
        let order_id = Uuid::new_v4().to_string();

        let now = chrono::Utc::now();
        let timestamp = Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        };

        // 计算订单总金额
        let mut total_amount = Money {
            currency_code: "USD".to_string(),
            units: 0,
            nanos: 0,
        };

        for item in &req.items {
            if let Some(price) = &item.unit_price {
                total_amount.units += price.units * item.quantity as i64;
                total_amount.nanos += price.nanos * item.quantity as i32;
            }
        }

        // 处理进位
        if total_amount.nanos >= 1_000_000_000 {
            total_amount.units += (total_amount.nanos / 1_000_000_000) as i64;
            total_amount.nanos %= 1_000_000_000;
        }

        let order = Order {
            id: order_id.clone(),
            user_id: req.user_id,
            items: req.items,
            total_amount: Some(total_amount),
            // status: OrderStatus::OrderStatusCreated.into(),
            status: OrderStatus::Created as i32, // 假设生成的枚举值是 Created 而不是 OrderStatusCreated
            created_at: Some(timestamp.clone()),
            updated_at: Some(timestamp),
            shipping_address: req.shipping_address,
        };

        self.orders.lock().unwrap().insert(order_id, order.clone());

        Ok(Response::new(CreateOrderResponse {
            status: Some(ApiStatus {
                code: 0,
                message: "Order created successfully".to_string(),
            }),
            order: Some(order),
        }))
    }

    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> Result<Response<GetOrderResponse>, Status> {
        let req = request.into_inner();
        let orders = self.orders.lock().unwrap();

        match orders.get(&req.order_id) {
            Some(order) => Ok(Response::new(GetOrderResponse {
                status: Some(ApiStatus {
                    code: 0,
                    message: "Order found".to_string(),
                }),
                order: Some(order.clone()),
            })),
            None => Err(Status::not_found("Order not found")),
        }
    }

    async fn list_user_orders(
        &self,
        request: Request<ListUserOrdersRequest>,
    ) -> Result<Response<ListUserOrdersResponse>, Status> {
        let req = request.into_inner();
        let pagination = req.pagination.unwrap_or(PaginationRequest {
            page: 1,
            page_size: 10,
        });

        let orders = self.orders.lock().unwrap();
        let mut user_orders: Vec<Order> = orders
            .values()
            .filter(|order| order.user_id == req.user_id)
            .cloned()
            .collect();

        // 按创建时间排序（最新的在前）
        user_orders.sort_by(|a, b| {
            let a_time = a.created_at.as_ref().map_or(0, |t| t.seconds);
            let b_time = b.created_at.as_ref().map_or(0, |t| t.seconds);
            b_time.cmp(&a_time)
        });

        let total_count = user_orders.len() as i32;

        // 简单分页
        let start = ((pagination.page - 1) * pagination.page_size) as usize;
        let end = std::cmp::min(start + pagination.page_size as usize, user_orders.len());
        let paged_orders = if start < user_orders.len() {
            user_orders[start..end].to_vec()
        } else {
            vec![]
        };

        Ok(Response::new(ListUserOrdersResponse {
            status: Some(ApiStatus {
                code: 0,
                message: "Orders retrieved successfully".to_string(),
            }),
            orders: paged_orders,
            total_count,
        }))
    }

    async fn update_order_status(
        &self,
        request: Request<UpdateOrderStatusRequest>,
    ) -> Result<Response<UpdateOrderStatusResponse>, Status> {
        let req = request.into_inner();
        let mut orders = self.orders.lock().unwrap();

        match orders.get_mut(&req.order_id) {
            Some(order) => {
                order.status = req.status;

                let now = chrono::Utc::now();
                let timestamp = Timestamp {
                    seconds: now.timestamp(),
                    nanos: now.timestamp_subsec_nanos() as i32,
                };
                order.updated_at = Some(timestamp);

                Ok(Response::new(UpdateOrderStatusResponse {
                    status: Some(ApiStatus {
                        code: 0,
                        message: "Order status updated successfully".to_string(),
                    }),
                    order: Some(order.clone()),
                }))
            }
            None => Err(Status::not_found("Order not found")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    // 共享数据存储
    let users = Arc::new(Mutex::new(HashMap::new()));
    let products = Arc::new(Mutex::new(HashMap::new()));
    let orders = Arc::new(Mutex::new(HashMap::new()));

    // 创建服务实例
    let user_service = UserServiceImpl {
        users: users.clone(),
    };

    let product_service = ProductServiceImpl {
        products: products.clone(),
    };

    let order_service = OrderServiceImpl {
        orders: orders.clone(),
        products: products.clone(),
    };

    println!("Server listening on {}", addr);

    // 启动服务器
    Server::builder()
        .add_service(user_service_server::UserServiceServer::new(user_service))
        .add_service(product_service_server::ProductServiceServer::new(
            product_service,
        ))
        .add_service(order_service_server::OrderServiceServer::new(order_service))
        .serve(addr)
        .await?;

    Ok(())
}
