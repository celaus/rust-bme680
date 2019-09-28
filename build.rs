use std::env;
use std::env::var;

use std::path::PathBuf;

const HEADER_FILE_NAME: &'static str = "BME680_driver/bme680.h";

fn main() {
    let project_dir = var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/BME680_driver/", project_dir);
    println!("cargo:rustc-link-lib=bme680");
    let bindings = bindgen::Builder::default()
        .header(HEADER_FILE_NAME)
        .clang_arg("-I/usr/arm-linux-gnueabihf/include/")
        .generate()
        .expect("Error generating bindings. Something failed in build.rs");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Error writing bindings");

    cc::Build::new()
        .file("BME680_driver/bme680.c")
        .pic(true)
        .shared_flag(true)
        .compile("bme680")
}
