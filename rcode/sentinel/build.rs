use std::{fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 告诉Cargo如果proto文件改变就重新运行

    let home = std::env::var("HOME").unwrap();
    println!("home: {}", home);
    let go_path = std::env::var("GOPATH").unwrap();
    println!("go path: {}", go_path);
    let mut file = File::create("build_output.log")?;
    writeln!(file, "home: {}", home)?;
    writeln!(file, "go path: {}", go_path)?;
    let out_dir = std::env::var("OUT_DIR").unwrap();
    writeln!(file, "out dir: {}", out_dir)?;
    writeln!(file, "cargo manifest dir: {}", env!("CARGO_MANIFEST_DIR"))?;

    // 设置proto文件的根目录路径
    // let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protos");
    let proto_root: PathBuf = PathBuf::from(go_path).join("src");

    // 设置proto文件的路径
    let baseinfo_v2_proto =
        proto_root.join("go.planetmeican.com/kiwi/grpc-proto/v2/baseinfo/baseinfo-v2.proto");

    // 设置include路径，使编译器能找到所有导入的proto文件
    // 添加protos目录作为include路径，这样可以解决不同目录下的proto文件导入问题
    let include_dirs = vec![proto_root.clone()];

    tonic_build::configure()
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[proto_root.join("go.planetmeican.com/maestro/bravo/meals/v1/meal.proto")],
            &include_dirs, // include路径
        )?;

    tonic_build::configure()
        .build_server(false)
        .extern_path(".meals.v1", "crate::proto::meals::v1") // 添加这一行
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[proto_root.join(
                "go.planetmeican.com/kiwi/grpc-proto/public/v1/baseinfo/baseinfo-public-v1.proto",
            )],
            &include_dirs, // include路径
        )?;

    tonic_build::configure()
        .build_server(false) // 只构建客户端代码
        .extern_path(".baseinfo.public.v1", "crate::proto::baseinfo_pub::v1")
        .extern_path(".meals.v1", "crate::proto::meals::v1")
        // 不设置out_dir，让tonic-build使用默认的输出目录
        // 启用类型属性，使生成的代码更符合Rust风格
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[baseinfo_v2_proto], // 编译的proto文件列表
            &include_dirs,        // include路径
        )?;

    Ok(())
}
