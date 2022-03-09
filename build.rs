use std::{env, path::PathBuf, process::Command};

const BINDINGS_FILE: &str = "bindings.rs";
const WRAPPER_FILE: &str = "wrapper.h";

fn main()
{
    let source = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let output = PathBuf::from(&env::var("OUT_DIR").unwrap());

    let name = "ucl";
    let include = output.join("include").display().to_string();
    let library = output.join("lib").display().to_string();

    // CFLAGS=-Wno-implicit-function-declaration
    println!("cargo:rustc-env=CFLAGS=-Wno-implicit-function-declaration");

    Command::new("./configure")
        .current_dir(&source.join("ucl-1.03"))
        .arg("--disable-debug")
        .arg("--disable-dependency-tracking")
        .arg(&format!("--prefix={}", output.display().to_string()))
        .output()
        .unwrap();

    Command::new("make")
        .current_dir(&source.join("ucl-1.03"))
        .arg("install")
        .output()
        .unwrap();

    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include))
        .header(WRAPPER_FILE)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    if cfg!(target_os = "windows") {
        builder = builder.generate_comments(false);
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(output.join(BINDINGS_FILE))
        .unwrap();

    println!("cargo:rustc-link-lib={}", name);
    println!("cargo:rustc-link-search={}", library);
    println!("cargo:include={}", include);
}
