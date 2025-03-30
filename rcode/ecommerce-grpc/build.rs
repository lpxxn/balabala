use std::io::Result;

fn main() -> Result<()> {
    // 编译所有的proto文件
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/pb") // 输出到src/pb目录
        .compile(
            &[
                "proto/user/service.proto",
                "proto/product/service.proto",
                "proto/order/service.proto",
            ],
            &["proto"], // 包含路径
        )?;
    Ok(())
}
