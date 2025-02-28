use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("../cyclonedds").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Adjust the include directory based on the build output
    let cyclone_include = dst.join("include");

    // Construct the full path to the header file.
    // let header_path = cyclone_include.join("ddsc").join("dds.h");

    // Invalidate the built crate when wrapper.h changes.
    println!("cargo:rerun-if-changed=wrapper.h");

    // Generate Rust bindings with bindgen.
    let bindings = bindgen::Builder::default()
        // Use the absolute path to the header file.
        .header("wrapper.h")
        // Pass the include path from cmake to clang.
        .clang_arg(format!("-I{}", cyclone_include.display()))
        .clang_arg(format!("-I{}/dds/cdr", cyclone_include.display()))
        .clang_arg(format!("-I{}/dds/ddsc", cyclone_include.display()))
        .clang_arg(format!("-I{}/dds/ddsi", cyclone_include.display()))
        .clang_arg(format!("-I{}/dds/ddsrt", cyclone_include.display()))
        .clang_arg(format!("-I{}/dds/dds_security", cyclone_include.display()))
        .clang_arg(format!("-I{}/dds", cyclone_include.display()))
        .clang_arg(format!("-I{}/idl", cyclone_include.display()))
        .clang_arg(format!("-I{}/idlc", cyclone_include.display()))
        .clang_arg(format!("-I{}/libidlc", cyclone_include.display()))
        // Add custom allow attributes:
        .raw_line("#[allow(non_upper_case_globals)]")
        .raw_line("#[allow(non_camel_case_types)]")
        .raw_line("#[allow(non_snake_case)]")
        // Automatically invalidate the build when any header changes.
        // Skip generating bindings for the problematic union:
        //.blocklist_item("ddsrt_log_cfg_union_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the generated bindings to $OUT_DIR/bindings.rs.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=cycloneddsidl");
    println!("cargo:rustc-link-lib=dds_security_ac");
    println!("cargo:rustc-link-lib=dds_security_auth");
    println!("cargo:rustc-link-lib=dds_security_crypto");
    println!("cargo:rustc-link-lib=ddsc");

}
