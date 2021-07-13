use std::env;

mod flags;

fn run_bindgen() {
    println!("Running bindgen...(not really)");

    println!("cwd: {:?}", env::current_dir());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    #[cfg(target_os = "windows")]
    let headers = "ndi/thirdparty/Include";

    #[cfg(target_os = "linux")]
    let headers = "ndi/thirdparty/Linux/Include";

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("ndi/wrapper.h")
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
    let mut binding_path = env::current_dir().unwrap();
    binding_path.push("ndi/src/internal");
    bindings
        .write_to_file(binding_path.join("bindings.rs"))
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
