use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=c/src/Number.cpp");
    println!("cargo:rerun-if-changed=c/src/Number.h");
    println!("cargo:rerun-if-changed=c/src/number_c.cpp");
    println!("cargo:rerun-if-changed=c/src/number_c.h");
    println!("cargo:rerun-if-changed=c/CMakeLists.txt");

    // Build the C++ library
    let mut build = cc::Build::new();
    
    build
        .cpp(true)
        .std("c++17")
        .include("c/src")
        .file("c/src/Number.cpp")
        .file("c/src/number_c.cpp");

    // Add compiler flags
    if cfg!(target_os = "windows") {
        build.flag("/W4");
        // On MSVC, we might need boost for uint128_t
        // This would need to be configured based on the system
    } else {
        build
            .flag("-Wall")
            .flag("-Wextra")
            .flag("-Wpedantic");
    }

    // Enable debug symbols if debug feature is enabled
    if cfg!(feature = "debug") {
        build.debug(true);
    }

    build.compile("xrpld-number");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("c/src/number_c.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}