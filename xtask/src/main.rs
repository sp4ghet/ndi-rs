use std::env;

mod flags;

fn run_bindgen() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    #[cfg(target_os = "windows")]
    let headers = "ndi/thirdparty/Windows/Include";

    #[cfg(target_os = "macos")]
    let headers = "ndi/thirdparty/Macos/Include";

    #[cfg(target_os = "linux")]
    let headers = "ndi/thirdparty/Linux/Include";

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("ndi/wrapper.h")
        .clang_args(["-I", headers].iter())
        .clang_arg("-fdeclspec")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let mut binding_path = env::current_dir().unwrap();
    binding_path.push("ndi/src/internal");

    #[cfg(target_os = "windows")]
    binding_path.push("bindings_windows.rs");

    #[cfg(target_os = "macos")]
    binding_path.push("bindings_macos.rs");

    #[cfg(target_os = "linux")]
    binding_path.push("bindings_linux.rs");

    bindings
        .write_to_file(binding_path)
        .expect("Couldn't write bindings!");
}

fn main() {
    match flags::Xtask::from_env() {
        Ok(x) => match x.subcommand {
            flags::XtaskCmd::Help(_) => {
                println!("{}", flags::Xtask::HELP)
            }
            flags::XtaskCmd::Bindgen(_) => run_bindgen(),
        },
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
