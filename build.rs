use std::{path::PathBuf, process::exit};
use walkdir::WalkDir;

fn main(){
    let proto_root = PathBuf::from("proto");
    let out_dir = PathBuf::from("src/pb"); // custom destination

    // Recursively collect all .proto files
    let protos: Vec<PathBuf> = WalkDir::new(&proto_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("proto"))
        .map(|e| e.into_path())
        .collect();

    // Configure prost-build to output into src/pb
    if let Err(error) = std::fs::create_dir("src/pb") && error.kind() != std::io::ErrorKind::AlreadyExists {
        eprintln!("Failed to create output directory: {:?}", error);
        exit(1);
    }
    let mut config = prost_build::Config::new();
    config.out_dir(out_dir);
    config.extern_path("._", "crate::websocket");

    config
        .compile_protos(
            &protos.iter().map(|p| p.as_path()).collect::<Vec<_>>(),
            &[proto_root], // include path
        )
        .map_err(|e| {
            eprintln!("Error compiling proto files: {:?}", e);
            eprint!("Don't forget to run: git submodule update --init --recursive");
        }).unwrap();
}
