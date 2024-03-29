use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search=C:\\Program Files\\LabJack\\Drivers");
    println!("cargo:rustc-link-search=C:\\Program Files (x86)\\LabJack\\Drivers\\64bit");
    println!("cargo:rustc-link-lib=LabJackM");

    let header = if std::env::var("DOCS_RS").is_ok() {
        "docs/wrapper.h"
    } else {
        "wrapper.h"
    };

    let bindings = bindgen::Builder::default()
        .header(header)
        .clang_arg("-IC:\\Program Files\\LabJack\\Drivers")
        .clang_arg("-IC:\\Program Files (x86)\\LabJack\\Drivers")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
