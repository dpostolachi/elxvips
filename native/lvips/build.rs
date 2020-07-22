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
    let out_paths: Vec<&str> = out_str.split( ' ' )
        .collect();
    let ( mut glib2_path, mut glib2_conf_path ) = (
        out_paths[0].to_string(),
        out_paths[1].to_string(),
    );

    glib2_path.push_str( "/" );
    glib2_conf_path.push_str( "/" );

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
        .clang_arg( &glib2_path )
        .clang_arg( &glib2_conf_path )
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .whitelist_function( "vips_error_buffer" )
        .whitelist_function( "vips_concurrency_set" )
        .whitelist_function( "vips_image_new_from_file" )
        .whitelist_function( "vips_image_new_from_buffer" )
        .whitelist_function( "vips_image_get_width" )
        .whitelist_function( "vips_image_get_height" )
        .whitelist_function( "vips_jpegsave" )
        .whitelist_function( "vips_pngsave" )
        .whitelist_function( "vips_jpegsave_buffer" )
        .whitelist_function( "vips_pngsave_buffer" )
        .whitelist_function( "vips_crop" )
        .whitelist_function( "vips_smartcrop" )
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