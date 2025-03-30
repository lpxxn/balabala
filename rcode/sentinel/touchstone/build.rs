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
    let proto_root: PathBuf = PathBuf::from(go_path).join("src");

    // 设置include路径，使编译器能找到所有导入的proto文件
    let include_dirs = vec![proto_root.clone()];

    // // 设置所有需要编译的proto文件路径
    // let proto_files = vec![
    //     //         // 元数据
    //     //         proto_root.join("go.planetmeican.com/nerds/proto/pbmeta/v1/pbmeta.proto"),
    //     // // 基础服务
    //     // proto_root.join("go.planetmeican.com/maestro/bravo/meals/v1/meal.proto"),

    //     proto_root.join("go.planetmeican.com/kiwi/grpc-proto/public/v1/baseinfo/baseinfo-public-v1.proto"),
    //     proto_root.join("go.planetmeican.com/kiwi/grpc-proto/v2/baseinfo/baseinfo-v2.proto"),

    //     proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/product.proto"),
    //     proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v5/data.proto"),
    //     proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/product.proto"),
    //     proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/backend/product.proto"),

    //     // // 用户中心
    //     // proto_root.join("go.planetmeican.com/api-center/protobufs/user-center/sso/v2/sso.proto"),

    //     // // 产品相关 v4

    //     // proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/menu_calendar.proto"),

    //     // // 产品相关 v5

    //     // proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/menu_calendar.proto"),

    //     // // 餐厅相关
    //     // proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v4/restaurant.proto"),
    //     // proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v5/restaurant.proto"),

    // ];

    // // 一次性配置并编译所有proto文件
    // tonic_build::configure()
    //     .build_server(false)
    //     .extern_path(".baseinfo.public.v1", "crate::proto::baseinfo_pub::v1")
    //     .extern_path(".meals.v1", "crate::proto::meals::v1")
    //     .extern_path(".pbmeta.v1", "crate::proto::pbmeta::v1")  // 取消注释这一行
    //     .extern_path(
    //         ".kiwi.baseinfo.cafeteria.v4",
    //         "crate::proto::baseinfo::cafeteria::v4",
    //     )
    //     .extern_path(
    //         ".kiwi.baseinfo.cafeteria.v5",
    //         "crate::proto::baseinfo::cafeteria::v5",
    //     )
    //     .extern_path(
    //         ".kiwi.baseinfo.product.v4",
    //         "crate::proto::baseinfo::product::v4",
    //     )
    //     .extern_path(
    //         ".kiwi.baseinfo.product.v5",
    //         "crate::proto::baseinfo::product::v5",
    //     )
    //     .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .compile_protos_with_config(tonic_config(), &proto_files, &include_dirs)?;

    // 检查proto文件是否存在
    let proto_files = vec![
        proto_root.join("go.planetmeican.com/kiwi/grpc-proto/v2/baseinfo/baseinfo-v2.proto"),
        // proto_root.join(
        //     "go.planetmeican.com/kiwi/grpc-proto/public/v1/baseinfo/baseinfo-public-v1.proto",
        // ),
        // proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/product.proto"),
        proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v5/data.proto"),
        // proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/product.proto"),
        proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/menu_calendar.proto",),
        proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/backend/product.proto"),
        proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v4/restaurant.proto"), 
        proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v5/restaurant.proto"),
    ];

    // 检查每个proto文件是否存在
    for proto_file in &proto_files {
        if !proto_file.exists() {
            println!(
                "cargo:warning=Proto file does not exist: {}",
                proto_file.display()
            );
            writeln!(file, "Proto file does not exist: {}", proto_file.display())?;
        } else {
            println!("cargo:warning=Proto file exists: {}", proto_file.display());
            writeln!(file, "Proto file exists: {}", proto_file.display())?;
        }
    }

    let mut normal_config = tonic_build::configure()
        .build_server(false)
        // .out_dir(&out_dir) // 明确设置输出目录
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    normal_config.clone().compile_protos(
        &[
            proto_root.join("go.planetmeican.com/maestro/bravo/meals/v1/meal.proto"),
            proto_root.join("go.planetmeican.com/nerds/proto/pbmeta/v1/pbmeta.proto"),
            proto_root.join(
                "go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/product.proto",
            ),
            proto_root.join(
                "go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/product.proto",
            ),
            proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/backend/product.proto"),

        ],
        &include_dirs,
    )?;

    // normal_config.clone().compile_protos(
    //     &[proto_root.join(
    //         "go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v5/menu_calendar.proto",

    //     ), proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v5/data.proto")],
    //     &include_dirs,
    // )?;
    // normal_config.clone().compile_protos(
    //     &[proto_root.join("go.planetmeican.com/nerds/proto/pbmeta/v1/pbmeta.proto")],
    //     &include_dirs,
    // )?;

    normal_config
        .clone()
        .extern_path(".meals.v1", "crate::proto::meals::v1")
        .compile_protos(
            &[proto_root.join(
                "go.planetmeican.com/kiwi/grpc-proto/public/v1/baseinfo/baseinfo-public-v1.proto",
            )],
            &include_dirs,
        )?;

    //     // // 产品相关 v4

    //     //
    match  normal_config
        .clone()
        .extern_path(".meals.v1", "crate::proto::meals::v1")
        // .extern_path(".kiwi.baseinfo.product.v4","crate::proto::baseinfo::product::v4",)
        // 添加类型自动实现
    .compile_well_known_types(true)
        .compile_protos(
            &[
                proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/menu_calendar.proto"),
            ],
            &[proto_root.clone(),
            proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4"),
            proto_root.join("go.planetmeican.com/api-center/protobufs"),
        ],
        ) {
            Ok(_) => {
                println!(
                    "cargo:warning=Successfully compiled: {}",
                    proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/menu_calendar.proto").display()
                );
                writeln!(file, "Successfully compiled: {}", proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/menu_calendar.proto").display())?;
            }
            Err(e) => {
                println!(
                    "cargo:warning=Failed to compile {}: {}",
                    proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/menu_calendar.proto").display(),
                    e
                );
                writeln!(file, "Failed to compile {}: {}", proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/product/v4/menu_calendar.proto").display(), e)?;
            }
        }

    // 一次性配置并编译所有proto文件
    let mut config = tonic_build::configure()
        .build_server(false)
        // .out_dir(&out_dir) // 明确设置输出目录
        .extern_path(".baseinfo.public.v1", "crate::proto::baseinfo_pub::v1")
        .extern_path(".meals.v1", "crate::proto::meals::v1")
        .extern_path(".pbmeta.v1", "crate::proto::pbmeta::v1")
        .extern_path(
            ".kiwi.baseinfo.cafeteria.v4",
            "crate::proto::baseinfo::cafeteria::v4",
        )
        .extern_path(
            ".kiwi.baseinfo.cafeteria.v5",
            "crate::proto::baseinfo::cafeteria::v5",
        )
        .extern_path(
            ".kiwi.baseinfo.product.v4",
            "crate::proto::baseinfo::product::v4",
        )
        .extern_path(
            ".kiwi.baseinfo.product.v5",
            "crate::proto::baseinfo::product::v5",
        )
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    // config.clone().compile_protos(
    //     &[
    //         proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v4/restaurant.proto"),
    //         proto_root.join("go.planetmeican.com/api-center/protobufs/kiwi/baseinfo/cafeteria/v5/restaurant.proto"),
    //         ],
    //     &include_dirs,
    // )?;
    // 尝试单独编译每个proto文件，以便更好地定位问题
    for proto_file in &proto_files {
        if proto_file.exists() {
            match config.clone().compile_protos(&[proto_file], &include_dirs) {
                Ok(_) => {
                    println!(
                        "cargo:warning=Successfully compiled: {}",
                        proto_file.display()
                    );
                    writeln!(file, "Successfully compiled: {}", proto_file.display())?;
                }
                Err(e) => {
                    println!(
                        "cargo:warning=Failed to compile {}: {}",
                        proto_file.display(),
                        e
                    );
                    writeln!(file, "Failed to compile {}: {}", proto_file.display(), e)?;
                }
            }
        }
    }

    // 列出生成的文件
    if let Ok(entries) = std::fs::read_dir(&out_dir) {
        writeln!(file, "Generated files in {}:", out_dir)?;
        for entry in entries {
            if let Ok(entry) = entry {
                writeln!(file, "  {}", entry.path().display())?;
                println!("cargo:warning=Generated file: {}", entry.path().display());
            }
        }
    }
    Ok(())
}
