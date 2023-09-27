use std::env;

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.

    let bindings = bindgen::Builder::default()
        // Use the lean.h specified in lakefile.lean
        .clang_arg(format!("-I{}", env::var("INCLUDE_PATH").unwrap()))
        // lean.h has several functions declared as `static inline` which we need to use
        .clang_arg("-fkeep-inline-functions")
        .header("src/bindings.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}