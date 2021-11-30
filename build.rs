use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:lib=ahrs");
    println!("cargo:rustc-link-lib=static=ahrs");

    cc::Build::new()
        .warnings(false)
        .cpp(true)
        .include("ahrs")
        .file("ahrs/nxp_matrix.cpp")
        .file("ahrs/nxp.cpp")
        .file("ahrs/wrapper.cpp")
        .compile("ahrs");

    println!("cargo:rerun-if-changed=ahrs/*");

    let bindings = bindgen::Builder::default()
        .header("ahrs/wrapper.hpp")
        .use_core()
        .ctypes_prefix("cty")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_eq(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
