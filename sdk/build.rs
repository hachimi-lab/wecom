use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/path/to/lib");
    println!("cargo:rustc-link-lib=bz2");

    bindgen::Builder::default()
        .header("src/archive.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings")
        .write_to_file(PathBuf::from("src/bindings.rs"))
        .expect("couldn't write bindings!");
}
