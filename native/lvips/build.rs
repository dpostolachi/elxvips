extern crate bindgen;
use std::process::Command;
use std::path::PathBuf;

fn main() {

    // Get clang arg to to include glib-2 from pkg-config command
    // should return something like: -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include
    let pkg_config_out = Command::new("sh")
        .arg("-c")
        .arg("pkg-config --cflags glib-2.0")
        .output()
        .unwrap()
        .stdout;

    let out_str = String::from_utf8_lossy( &pkg_config_out );
    let flags: Vec<&str> = out_str.split( ' ' )
        .map( | part | part.trim() )
        .collect();

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=vips");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=lib/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings_default = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("lib/wrapper.h");
    
    for flag in flags {
        bindings_default = bindings_default
            .clang_arg( flag );
    }

    let bindings = bindings_default
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .whitelist_function( "vips_error_buffer" )
        .whitelist_function( "vips_error_clear" )
        .whitelist_function( "vips_concurrency_set" )
        .whitelist_function( "vips_image_new_from_file" )
        .whitelist_function( "vips_image_new_from_buffer" )
        .whitelist_function( "vips_image_get_width" )
        .whitelist_function( "vips_image_get_height" )
        .whitelist_function( "vips_jpegsave" )
        .whitelist_function( "vips_pngsave" )
        .whitelist_function( "vips_webpsave" )
        .whitelist_function( "vips_jpegsave_buffer" )
        .whitelist_function( "vips_pngsave_buffer" )
        .whitelist_function( "vips_webpsave_buffer" )
        .whitelist_function( "vips_crop" )
        .whitelist_function( "vips_smartcrop" )
        .whitelist_function( "vips_array_double_new" )
        .whitelist_function( "vips_resize" )
        .whitelist_function( "vips_image_get_as_string" )
        .whitelist_function( "g_object_unref" )
        .whitelist_function( "g_free()" )
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