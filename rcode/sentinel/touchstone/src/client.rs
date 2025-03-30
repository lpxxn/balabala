//! gRPC客户端实现

use crate::proto::baseinfo::v2::{
    QueryCafeteriaDetailRequest, QueryCafeteriasRequest, QuerySimpleCafeteriasRequest,
    RestaurantServiceClient,
};
use crate::proto::pbmeta::v1::QueryByIdRequest;
use std::error::Error;
use std::time::Duration;
use tonic::transport::{Channel, Endpoint};

/// 餐厅服务客户端封装
pub struct RestaurantClient {
    client: RestaurantServiceClient<Channel>,
}

impl RestaurantClient {
    /// 创建新的客户端连接
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn Error>> {
        // 配置连接参数
        let endpoint = Endpoint::from_shared(addr.to_string())?
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .tcp_keepalive(Some(Duration::from_secs(30)));

        // 建立连接
        let channel = endpoint.connect().await?;
        let client = RestaurantServiceClient::new(channel);

        Ok(Self { client })
    }

    /// 查询食堂列表
    pub async fn query_cafeterias(
        &mut self,
        name: Option<String>,
        limit: i32,
    ) -> Result<(), Box<dyn Error>> {
        let request = tonic::Request::new(QueryCafeteriasRequest {
            name: name.unwrap_or_default(),
            limit,
            next_snowflake_id: 0,
        });

        println!("发送查询食堂请求...");
        let response = self.client.query_cafeterias(request).await?;
        let response = response.into_inner();

        println!("查询成功! 元数据: {:?}", response.meta);
        println!("找到 {} 个食堂:", response.data.len());

        for (i, cafeteria) in response.data.iter().enumerate() {
            println!(
                "{}: ID={}, 名称={}, 雪花ID={}",
                i + 1,
                cafeteria.id,
                cafeteria.name,
                cafeteria.snowflake_id
            );
        }

        println!("下一个雪花ID: {}", response.next_snowflake_id);

        Ok(())
    }

    /// 根据ID查询餐厅
    pub async fn query_restaurant_by_id(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let request = tonic::Request::new(QueryByIdRequest { id });

        println!("发送查询餐厅详情请求, ID: {}...", id);
        let response = self.client.query_restaurant_by_id(request).await?;
        let response = response.into_inner();

        println!("查询成功! 元数据: {:?}", response.meta);
        if let Some(restaurant) = response.data {
            println!(
                "餐厅信息: ID={}, 名称={}, 雪花ID={}",
                restaurant.id, restaurant.name, restaurant.snowflake_id
            );
        } else {
            println!("未找到餐厅信息");
        }

        Ok(())
    }

    /// 根据雪花ID列表查询简单食堂信息
    pub async fn query_simple_cafeterias(
        &mut self,
        snowflake_ids: Vec<i64>,
    ) -> Result<(), Box<dyn Error>> {
        let request = tonic::Request::new(QuerySimpleCafeteriasRequest {
            snowflake_i_ds: snowflake_ids,
            cafeteria_i_ds: vec![],
        });

        println!("发送查询简单食堂信息请求...");
        let response = self.client.query_simple_cafeterias(request).await?;
        let response = response.into_inner();

        println!("查询成功! 元数据: {:?}", response.meta);
        println!("找到 {} 个食堂:", response.data.len());

        for (i, cafeteria) in response.data.iter().enumerate() {
            println!(
                "{}: ID={}, 名称={}, 雪花ID={}, 城市ID={}",
                i + 1,
                cafeteria.id,
                cafeteria.name,
                cafeteria.snowflake_id,
                cafeteria.city_id
            );
        }

        Ok(())
    }

    /// 查询食堂详情
    pub async fn query_cafeteria_detail(
        &mut self,
        snowflake_id: i64,
    ) -> Result<(), Box<dyn Error>> {
        let request = tonic::Request::new(QueryCafeteriaDetailRequest {
            cafeteria_snowflake_id: snowflake_id,
        });

        println!("发送查询食堂详情请求, 雪花ID: {}...", snowflake_id);
        let response = self
            .client
            .query_cafeteria_detail_by_snowflake_id(request)
            .await?;
        let response = response.into_inner();

        println!("查询成功! 元数据: {:?}", response.meta);
        if let Some(detail) = response.data {
            println!("食堂详情: ");
            println!("  基本信息: ID={}, 名称={}", detail.id, detail.name);
            println!("  地址: {}", detail.address);
            println!("  档口数量: {}", detail.restaurant_count);
            println!("  管理员数量: {}", detail.manager_count);
        } else {
            println!("未找到食堂详情");
        }

        Ok(())
    }
}
