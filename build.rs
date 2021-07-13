extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    // TODO: find a better path to this stuff
    Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps")
}

#[cfg(target_os = "windows")]
fn win_link_and_load() {
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

#[cfg(target_os = "linux")]
fn lin_link_and_load() {
    println!("cargo:rustc-link-lib=ndi",);
    let mut lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    lib_path.push("thirdparty/Linux/Lib");
    println!(
        "cargo:rustc-link-search={}",
        lib_path.to_str().unwrap().to_string()
    );

    // copy dll to OUT_DIR
    let out_path = get_output_path();
    let src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("thirdparty/Linux/Lib/libndi.so.4");
    let dst = Path::join(&out_path, "libndi.so.4");
    std::fs::copy(src, dst).unwrap();
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
    #[cfg(target_os = "windows")]
    let headers = "./thirdparty/Include";

    #[cfg(target_os = "linux")]
    let headers = "./thirdparty/Linux/Include";

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(["-I", headers].iter())
        .clang_arg("-fdeclspec")
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

    #[cfg(target_os = "windows")]
    win_link_and_load();

    #[cfg(target_os = "linux")]
    lin_link_and_load();
}
