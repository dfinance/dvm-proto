// see https://doc.rust-lang.org/cargo/reference/build-scripts.html
extern crate tonic_build;

const OUT_DIR: &str = "src/grpc";

const PB_PATH: [&str; 5] = [
    "../proto/dfinance/dvm/common-types.proto",
    "../proto/dfinance/dvm/compiler.proto",
    "../proto/dfinance/dvm/data-source.proto",
    "../proto/dfinance/dvm/metadata.proto",
    "../proto/dfinance/dvm/vm.proto",
];

const BASE_PATH: &str = "../proto";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/build.rs");
    tonic_build::configure()
        .out_dir(OUT_DIR)
        .compile(&PB_PATH, &[BASE_PATH])?;
    Ok(())
}
