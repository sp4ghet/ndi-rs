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
    #[cfg(target_os = "windows")]
    win_link_and_load();

    #[cfg(target_os = "linux")]
    lin_link_and_load();
}
