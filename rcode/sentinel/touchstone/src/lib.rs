//! 生成的protobuf代码模块和客户端实现
// 导出客户端模块
pub mod client;

// 包含从proto文件生成的代码
pub mod proto {

    pub mod meals {
        pub mod v1 {
            tonic::include_proto!("meals.v1");
        }
    }
    // 导入公共定义
    pub mod pbmeta {
        pub mod v1 {
            tonic::include_proto!("pbmeta.v1");
        }
    }
    pub mod baseinfo_pub {
        pub mod v1 {
            tonic::include_proto!("baseinfo.public.v1");
        }
    }
    // 导入baseinfo.v2服务
    pub mod baseinfo {
        pub mod v2 {
            // 包含从baseinfo-v2.proto生成的代码
            tonic::include_proto!("baseinfo.v2");

            // 重新导出服务客户端，方便使用
            pub use restaurant_service_client::RestaurantServiceClient;
        }
    }
}
