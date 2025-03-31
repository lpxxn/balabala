use std::io::Result;

fn main() -> Result<()> {
    // 创建输出目录
    std::fs::create_dir_all("src/pb")?;

    println!("cargo:rerun-if-changed=proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/pb")
        // 添加这个选项，确保所有类型都被生成
        .compile_well_known_types(true)
        .extern_path(".common", "crate::pb::common")
        .extern_path(".user", "crate::pb::user")
        .extern_path(".product", "crate::pb::product")
        .extern_path(".payment", "crate::pb::payment")
        .compile(
            &[
                // 包含所有相关的proto文件
                "proto/common/base.proto",
                "proto/user/model.proto",
                "proto/user/service.proto",
                "proto/product/model.proto",
                "proto/product/service.proto",
                "proto/payment/model.proto",
                "proto/payment/service.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
