use std::{fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME").unwrap();
    println!("home: {}", home);
    let go_path = std::env::var("GOPATH").unwrap();
    println!("go path: {}", go_path);

    // 设置proto文件的根目录路径
    // let proto_root: PathBuf = PathBuf::from(go_path).join("src");
    let proto_root: PathBuf = PathBuf::from(home).join("go/src");

    // 设置include路径，使编译器能找到所有导入的proto文件
    let include_dirs = vec![proto_root.clone()];

    // 检查proto文件是否存在
    let proto_files = vec![
            proto_root.join("go.planetmeican.com/kiwi/grpc-proto/v2/baseinfo/baseinfo-v2.proto"),
            proto_root.join("go.planetmeican.com/kiwi/grpc-proto/public/v1/baseinfo/baseinfo-public-v1.proto"),
            proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/product.proto"),
            proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v5/data.proto"),
            proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/product.proto"),
            proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/backend/product.proto"),
        ];

    // 检查每个proto文件是否存在
    for proto_file in &proto_files {
        if !proto_file.exists() {
            println!(
                "cargo:warning=Proto file does not exist: {}",
                proto_file.display()
            );
            println!("Proto file does not exist: {}", proto_file.display());
        } else {
            println!("cargo:warning=Proto file exists: {}", proto_file.display());
            println!("Proto file exists: {}", proto_file.display());
        }
    }

    Ok(())
}
