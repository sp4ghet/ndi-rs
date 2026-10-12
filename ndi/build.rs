use std::{env, fs, path::Path};

fn copy_library(manifest: &Path, output: &Path, source: &str, name: &str) {
    let source = manifest.join(source);
    println!("cargo:rerun-if-changed={}", source.display());
    fs::copy(&source, output.join(name))
        .unwrap_or_else(|error| panic!("failed to copy {}: {error}", source.display()));
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    assert!(
        arch == "x86_64" && matches!(os.as_str(), "windows" | "linux" | "macos"),
        "the bundled NDI runtimes and bindings support only x86_64 Windows, Linux, and macOS; target is {arch}-{os}"
    );
    assert!(
        (os != "windows" || target_env == "msvc") && (os != "linux" || target_env == "gnu"),
        "the bundled NDI libraries require Windows MSVC or Linux glibc"
    );
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output = env::var("OUT_DIR").unwrap();
    let manifest = Path::new(&manifest);
    let output = Path::new(&output);

    // Cargo adds library search paths inside OUT_DIR to the loader path for
    // `cargo run` and `cargo test`, including downstream workspace packages.
    println!("cargo:rustc-link-search=native={}", output.display());
    match os.as_str() {
        "windows" => {
            copy_library(
                manifest,
                output,
                "thirdparty/Windows/Lib/Processing.NDI.Lib.x64.lib",
                "Processing.NDI.Lib.x64.lib",
            );
            copy_library(
                manifest,
                output,
                "thirdparty/Windows/Bin/Processing.NDI.Lib.x64.dll",
                "Processing.NDI.Lib.x64.dll",
            );
            println!("cargo:rustc-link-lib=dylib=Processing.NDI.Lib.x64");
        }
        "linux" => {
            // Use the actual file, including on checkouts without symlinks.
            for name in ["libndi.so", "libndi.so.4"] {
                copy_library(
                    manifest,
                    output,
                    "thirdparty/Linux/Lib/libndi.so.4.6.2",
                    name,
                );
            }
            println!("cargo:rustc-link-lib=dylib=ndi");
        }
        "macos" => {
            // -lndi needs the unversioned name; dyld needs the install name.
            for name in ["libndi.dylib", "libndi.4.dylib"] {
                copy_library(
                    manifest,
                    output,
                    "thirdparty/Macos/Lib/libndi.4.dylib",
                    name,
                );
            }
            println!("cargo:rustc-link-lib=dylib=ndi");
        }
        _ => unreachable!(),
    }
}
