use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");
    cc::Build::new()
        .warnings(false)
        .include("ahrs")
        .file("ahrs/Adafruit_AHRS_NXPmatrix.c")
        .file("ahrs/Adafruit_AHRS_NXPFusion.cpp")
        .file("ahrs/wrapper.c")
        .compile("ahrs");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=ahrs/wrapper.h");

    // cxx_build::bridge("src/ffi.rs")
    //     .include("ahrs")
    //     .file("ahrs/Adafruit_AHRS_NXPFusion.cpp")
    //     .flag_if_supported("-std=c++14")
    //     .compile("ahrs");

    // println!("cargo:rerun-if-changed=src/main.rs");
    // println!("cargo:rerun-if-changed=src/blobstore.cc");
    // println!("cargo:rerun-if-changed=include/blobstore.h");
}
