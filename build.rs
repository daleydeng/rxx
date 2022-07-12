extern crate cc;
use std::env;

fn main() {
    let prefix = env::var("CONDA_PREFIX").unwrap();

    cc::Build::new()
	.file("src/ffi.cc")
	.cpp(true)
        .flag_if_supported("-std=c++14")
        .include("include")
	.include(prefix.clone() + "/include")
        .compile("rxx");

    println!("cargo:rerun-if-changed=src/ffi.cc");
    println!("cargo:rerun-if-changed=include/wrapper.hh");
}
