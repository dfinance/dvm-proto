// see https://doc.rust-lang.org/cargo/reference/build-scripts.html
extern crate tonic_build;

use std::path::Path;
use std::fs::canonicalize;

const OUT_DIR: &str = "src/grpc";
const PB_PATH: [&str; 5] = [
    "../protos/vm.proto",
    "../protos/common-types.proto",
    "../protos/compiler.proto",
    "../protos/metadata.proto",
    "../protos/data-source.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/build.rs");

    let protos = canonicalize("../protos/")
        .map_err(|err| eprintln!("invalid path: {:?}", err))
        .unwrap();
    eprintln!("canonicalized protos-sources path: {:?}", protos);

    for path in PB_PATH.iter() {
        println!("rerun-if-changed={}", path);
        println!("cargo:rerun-if-changed={}", path);

        let mut sourcefile = protos.clone();
        sourcefile.push(path);
        eprintln!("sourcefile path: {:?}", sourcefile);

        let proto_path: &Path = path.as_ref();
        let proto_dir = proto_path
            .parent()
            .expect("proto file should reside in a directory");
        tonic_build::configure()
            .out_dir(OUT_DIR)
            .compile(&[proto_path], &[proto_dir])?
    }

    Ok(())
}
