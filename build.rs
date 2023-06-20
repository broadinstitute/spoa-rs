use std::path::PathBuf;
use cmake;
use cxx_build::CFG;
use std::fs::canonicalize;


fn main() {

    let out_dir = cmake::Config::new("spoa")
        .uses_cxx11()
        .define("spoa_install", "OFF")
        .define("spoa_build_exe", "OFF")
        .define("spoa_build_tests", "OFF")
        .build_target("spoa")
        .build();

    println!("cargo:rustc-link-search=native={}/build/lib", out_dir.display());
    println!("cargo:rustc-link-lib=spoa");

    let spoa_include = canonicalize(PathBuf::from("spoa/include")).unwrap();
    CFG.exported_header_dirs.push(&spoa_include);

    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .file("cxx/spoa_rs.cpp")
        .flag_if_supported("-std=c++14")
        .compile("spoa_rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cxx/spoa_rs.hpp");
    println!("cargo:rerun-if-changed=cxx/spoa_rs.cpp");
    println!("cargo:rerun-if-changed=spoa/spoa.hpp");
}