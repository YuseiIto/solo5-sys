extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=binding.h");

    let bindings = bindgen::Builder::default()
        .header("binding.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-Isolo5/include")
        .ctypes_prefix("cty")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
