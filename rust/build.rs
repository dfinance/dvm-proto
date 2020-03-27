// see https://doc.rust-lang.org/cargo/reference/build-scripts.html
extern crate tonic_build;

use std::path::Path;
use std::fs::canonicalize;

const OUT_DIR: &str = "src/grpc";
const PB_DIR: &str = "../protos/";
const PB_PATH: [&str; 2] = [
    "vm.proto",
    "data-source.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/build.rs");

	 let protos = canonicalize(PB_DIR).map_err(|err| eprintln!("ERROR: protos-sources: {:?}", err)).unwrap();
	 eprintln!("canonicalized protos-sources path: {:?}", protos);
	 std::fs::read_dir(&protos).map(|res|res.for_each(|item| eprintln!("{:?}", item))).unwrap();

    for path in PB_PATH.iter() {
        println!("rerun-if-changed={}", path);
        println!("cargo:rerun-if-changed={}", path);
        eprintln!("rel path: {}", path);
        let mut sourcefile = protos.clone();
        sourcefile.push(path);
		  eprintln!("~ path: {:?}", sourcefile);
		  let sourcefile = canonicalize(&sourcefile).unwrap();
        eprintln!("canonicalized path: {:?}", &sourcefile);
        let proto_path: &Path = sourcefile.as_ref();
        let proto_dir = proto_path
            .parent()
            .expect("proto file should reside in a directory");
        tonic_build::configure()
            .out_dir(OUT_DIR)
            .compile(&[proto_path], &[proto_dir])?
    }

    Ok(())
}
