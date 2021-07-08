extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=ndi");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(["-I", "./thirdparty/Include"].iter())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let mut binding_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    binding_path.push("src/internal");
    bindings
        .write_to_file(binding_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // tell cargo where the static lib is
    println!("cargo:rustc-link-lib=Processing.NDI.Lib.x64");
    let mut lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    lib_path.push("thirdparty\\Lib");
    println!(
        "cargo:rustc-link-search={}",
        lib_path.to_str().unwrap().to_string()
    );

    // copy dll to OUT_DIR

    let out_path = get_output_path();
    let src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("thirdparty\\Bin\\Processing.NDI.Lib.x64.dll");
    let dst = Path::join(&out_path, "Processing.NDI.Lib.x64.dll");
    std::fs::copy(src, dst).unwrap();
}
