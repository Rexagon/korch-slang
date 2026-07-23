use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").expect("TARGET is not set");

    println!("cargo:rerun-if-changed=src/slang.h");

    let bindings = bindgen::Builder::default()
        .header("src/slang.h")
        .allowlist_type("Slang.*")
        .clang_args(["-x", "c++"])
        .clang_arg("-std=c++20")
        .clang_arg(format!("--target={target}"))
        .enable_cxx_namespaces()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
