extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=vips");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=lib/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("lib/wrapper.h")
        .clang_arg( "-I/usr/local/include/glib-2.0" )
        .clang_arg( "-I/usr/local/lib/glib-2.0/include/" )
        .clang_arg( "-I/usr/include/glib-2.0" )
        .clang_arg( "-I/usr/lib/glib-2.0/include/" )
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .whitelist_function( "vips_error_buffer" )
        .whitelist_function( "vips_concurrency_set" )
        .whitelist_function( "vips_image_new_from_file" )
        .whitelist_function( "vips_image_get_width" )
        .whitelist_function( "vips_image_get_height" )
        .whitelist_function( "vips_jpegsave" )
        .whitelist_function( "vips_pngsave" )
        .whitelist_function( "vips_jpegsave_buffer" )
        .whitelist_function( "vips_pngsave_buffer" )
        .whitelist_function( "vips_crop" )
        .whitelist_function( "vips_array_double_new" )
        .whitelist_function( "vips_resize" )
        .whitelist_var( "VipsInterpretation_VIPS_INTERPRETATION_XYZ" )
        .whitelist_type( "_VipsImage" )
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from( "lib" );
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}