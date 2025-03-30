use std::error::Error;
use touchstone::client::RestaurantClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 服务器地址 - 需要替换为实际的gRPC服务器地址
    let server_addr = "http://[::1]:10240";

    println!("连接到gRPC服务器: {}", server_addr);
    let mut client = match RestaurantClient::connect(server_addr).await {
        Ok(client) => {
            println!("连接成功!");
            client
        }
        Err(err) => {
            println!("连接失败: {}", err);
            return Err(err);
        }
    };

    // 示例1: 查询食堂列表
    println!("\n=== 示例1: 查询食堂列表 ===");
    if let Err(err) = client.query_cafeterias(None, 10).await {
        println!("查询食堂列表失败: {}", err);
    }

    // 示例2: 根据ID查询餐厅
    println!("\n=== 示例2: 根据ID查询餐厅 ===");
    // 注意: 需要替换为实际存在的ID
    if let Err(err) = client.query_restaurant_by_id(1).await {
        println!("查询餐厅详情失败: {}", err);
    }

    // 示例3: 根据雪花ID列表查询简单食堂信息
    println!("\n=== 示例3: 根据雪花ID列表查询简单食堂信息 ===");
    // 注意: 需要替换为实际存在的雪花ID
    if let Err(err) = client.query_simple_cafeterias(vec![1, 2, 3]).await {
        println!("查询简单食堂信息失败: {}", err);
    }

    // 示例4: 查询食堂详情
    println!("\n=== 示例4: 查询食堂详情 ===");
    // 注意: 需要替换为实际存在的雪花ID
    if let Err(err) = client.query_cafeteria_detail(1).await {
        println!("查询食堂详情失败: {}", err);
    }

    println!("\n所有示例执行完毕");
    Ok(())
}
