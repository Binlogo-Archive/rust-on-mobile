extern crate cbindgen;
use cbindgen::Language::C;
use std::env;

fn main() {
    setup_cbindgen();
}

fn setup_cbindgen() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(C)
        .with_include("starwars_core.h")
        .generate()
        .expect("Unable to generate binging")
        .write_to_file("src/ffi/starwars.h");
}
