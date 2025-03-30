use tonic::{Request, transport::Channel};
use uuid::Uuid;

// 导入生成的protobuf模块
mod pb;
use pb::common::*;
use pb::order::*;
use pb::product::*;
use pb::user::*;

use pb::common::Status as ApiStatus;

use order_service_client::OrderServiceClient;
use product_service_client::ProductServiceClient;
use user_service_client::UserServiceClient;

async fn run_user_service_demo(
    mut client: UserServiceClient<Channel>,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n=== User Service Demo ===");

    // 创建用户
    let address = Address {
        street_address: "123 Main St".to_string(),
        city: "New York".to_string(),
        state: "NY".to_string(),
        country: "USA".to_string(),
        zip_code: "10001".to_string(),
    };

    let create_user_req = Request::new(CreateUserRequest {
        username: "johndoe".to_string(),
        email: "john.doe@example.com".to_string(),
        password: "secure_password".to_string(),
        full_name: "John Doe".to_string(),
        address: Some(address),
    });

    let response = client.create_user(create_user_req).await?;
    let user = response.into_inner().user.unwrap();
    println!("Created user: {:#?}", user);

    // 获取用户
    let get_user_req = Request::new(GetUserRequest {
        user_id: user.id.clone(),
    });

    let response = client.get_user(get_user_req).await?;
    println!("Retrieved user: {:#?}", response.into_inner().user.unwrap());

    // 列出用户
    let list_users_req = Request::new(ListUsersRequest {
        pagination: Some(PaginationRequest {
            page: 1,
            page_size: 10,
        }),
    });

    let response = client.list_users(list_users_req).await?;
    println!("Listed users: {:#?}", response.into_inner().users);

    Ok(user.id)
}

async fn run_product_service_demo(
    mut client: ProductServiceClient<Channel>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("\n=== Product Service Demo ===");

    let mut product_ids = Vec::new();

    // 创建产品1
    let price1 = Money {
        currency_code: "USD".to_string(),
        units: 599,
        nanos: 990000000,
    };

    // 修复枚举值名称 - 使用正确的生成枚举值
    // 通常生成的枚举值会是 CATEGORY_ELECTRONICS 而不是 CategoryElectronics
    let create_product_req1 = Request::new(CreateProductRequest {
        name: "Smartphone X".to_string(),
        description: "Latest smartphone with amazing features".to_string(),
        price: Some(price1),
        // category: 1, // 使用数字值 1 代表 CATEGORY_ELECTRONICS
        category: Category::Electronics as i32,
        stock_quantity: 100,
        image_urls: vec!["https://example.com/smartphone_x.jpg".to_string()],
    });

    let response = client.create_product(create_product_req1).await?;
    let product1 = response.into_inner().product.unwrap();
    println!("Created product 1: {:#?}", product1);
    product_ids.push(product1.id.clone());

    // 创建产品2
    let price2 = Money {
        currency_code: "USD".to_string(),
        units: 29,
        nanos: 990000000,
    };

    let create_product_req2 = Request::new(CreateProductRequest {
        name: "T-shirt".to_string(),
        description: "Comfortable cotton t-shirt".to_string(),
        price: Some(price2),
        // category: 2, // 使用数字值 2 代表 CATEGORY_CLOTHING
        category: Category::Clothing as i32,
        stock_quantity: 200,
        image_urls: vec!["https://example.com/tshirt.jpg".to_string()],
    });

    let response = client.create_product(create_product_req2).await?;
    let product2 = response.into_inner().product.unwrap();
    println!("Created product 2: {:#?}", product2);
    product_ids.push(product2.id.clone());

    // 获取产品
    let get_product_req = Request::new(GetProductRequest {
        product_id: product1.id.clone(),
    });

    let response = client.get_product(get_product_req).await?;
    println!(
        "Retrieved product: {:#?}",
        response.into_inner().product.unwrap()
    );

    // 列出产品
    let list_products_req = Request::new(ListProductsRequest {
        pagination: Some(PaginationRequest {
            page: 1,
            page_size: 10,
        }),
        category: 0, // 所有类别
    });

    let response = client.list_products(list_products_req).await?;
    println!("Listed products: {:#?}", response.into_inner().products);

    Ok(product_ids)
}

async fn run_order_service_demo(
    mut client: OrderServiceClient<Channel>,
    user_id: String,
    product_ids: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Order Service Demo ===");

    // 创建订单项
    let order_item1 = OrderItem {
        product_id: product_ids[0].clone(),
        product_name: "Smartphone X".to_string(),
        quantity: 1,
        unit_price: Some(Money {
            currency_code: "USD".to_string(),
            units: 599,
            nanos: 990000000,
        }),
    };

    let order_item2 = OrderItem {
        product_id: product_ids[1].clone(),
        product_name: "T-shirt".to_string(),
        quantity: 2,
        unit_price: Some(Money {
            currency_code: "USD".to_string(),
            units: 29,
            nanos: 990000000,
        }),
    };

    // 创建订单
    let shipping_address = Address {
        street_address: "456 Market St".to_string(),
        city: "San Francisco".to_string(),
        state: "CA".to_string(),
        country: "USA".to_string(),
        zip_code: "94103".to_string(),
    };

    let create_order_req = Request::new(CreateOrderRequest {
        user_id: user_id.clone(),
        items: vec![order_item1, order_item2],
        shipping_address: Some(shipping_address),
    });

    let response = client.create_order(create_order_req).await?;
    let order = response.into_inner().order.unwrap();
    println!("Created order: {:#?}", order);

    // 获取订单
    let get_order_req = Request::new(GetOrderRequest {
        order_id: order.id.clone(),
    });

    let response = client.get_order(get_order_req).await?;
    println!(
        "Retrieved order: {:#?}",
        response.into_inner().order.unwrap()
    );

    // 列出用户订单
    let list_orders_req = Request::new(ListUserOrdersRequest {
        user_id: user_id.clone(),
        pagination: Some(PaginationRequest {
            page: 1,
            page_size: 10,
        }),
    });

    let response = client.list_user_orders(list_orders_req).await?;
    println!("Listed user orders: {:#?}", response.into_inner().orders);

    // 更新订单状态
    let update_status_req = Request::new(UpdateOrderStatusRequest {
        order_id: order.id.clone(),
        // status: 2, // 使用数字值 2 代表 ORDER_STATUS_PROCESSING
        status: OrderStatus::Processing as i32,
    });

    let response = client.update_order_status(update_status_req).await?;
    println!(
        "Updated order status: {:#?}",
        response.into_inner().order.unwrap()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到服务器
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    // 创建客户端
    let user_client = UserServiceClient::new(channel.clone());
    let product_client = ProductServiceClient::new(channel.clone());
    let order_client = OrderServiceClient::new(channel.clone());

    // 运行用户服务演示
    let user_id = run_user_service_demo(user_client).await?;

    // 运行产品服务演示
    let product_ids = run_product_service_demo(product_client).await?;

    // 运行订单服务演示
    run_order_service_demo(order_client, user_id, product_ids).await?;

    println!("\nAll demos completed successfully!");
    Ok(())
}
