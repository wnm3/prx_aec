use std::env;
use std::path::{Path, PathBuf};

use cmake::Config;

fn copy_folder(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).expect("Failed to create dst directory");
    if cfg!(unix) {
        std::process::Command::new("cp")
            .arg("-rf")
            .arg(src)
            .arg(dst.parent().unwrap())
            .status()
            .expect("Failed to execute cp command");
    }

    if cfg!(windows) {
        std::process::Command::new("robocopy.exe")
            .arg("/e")
            .arg(src)
            .arg(dst)
            .status()
            .expect("Failed to execute robocopy command");
    }
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_src = Path::new(&manifest_dir).join("speexdsp");
    let lib_dst = out_dir.join("speexdsp");
    let profile = env::var("SPEEXDSP_LIB_PROFILE").unwrap_or("Release".to_string());
    if !lib_dst.exists() {
        copy_folder(&lib_src, &lib_dst);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", lib_dst.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");
    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(bindings_path)
        .expect("Failed to write bindings");

    let mut config = Config::new(&lib_dst);

    let build_dir = config.profile(&profile).build();
    println!(
        "cargo:rustc-link-search={}",
        build_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=speexdsp");
}
