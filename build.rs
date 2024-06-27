use std::path::PathBuf;

fn main() {
    let name = "NET_ICube_API.h";
    let target_triple = std::env::var("TARGET").unwrap();

    // if not windows, do nothing
    if !cfg!(target_os = "windows") {
        return;
    }

    println!("cargo:rerun-if-changed={}", name);

    let bindings = bindgen::Builder::default()
        .header(name)
        .clang_args([
            //"-target", "stable-x86_64-pc-windows-msvc",
            "-target", &target_triple,
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join(format!("{}.rs", name)))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=NET_ICube_API.cpp");
    cc::Build::new()
        .file("NET_ICube_API.cpp")
        .compile("icube_init");
}