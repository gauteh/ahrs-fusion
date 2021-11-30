#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(deref_nullptr)] // https://github.com/rust-lang/rust-bindgen/issues/1651
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
