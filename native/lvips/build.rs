extern crate bindgen;
use std::process::Command;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
fn extra_build_steps() {
    // link brew libraries
    println!(r"cargo:rustc-link-search=/opt/homebrew/lib");
}

#[cfg(not(target_os = "macos"))]
fn extra_build_steps() {
}

fn main() {

    // Get vips dependencies
    // should return something like: -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include ...
    let pkg_config_out = Command::new("sh")
        .arg("-c")
        .arg("pkg-config --cflags --libs vips glib-2.0")
        .output()
        .unwrap()
        .stdout;

    let out_str = String::from_utf8_lossy( &pkg_config_out );
    let flags: Vec<&str> = out_str.split( ' ' )
        .map( | part | part.trim() )
        .collect();
    
    // extra steps for platform
    extra_build_steps();
    
    // Tell cargo to tell rustc to link the system vips and glib-2.0
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
        .allowlist_function( "vips_error_buffer" )
        .allowlist_function( "vips_error_clear" )
        .allowlist_function( "vips_concurrency_set" )
        .allowlist_function( "vips_image_new_from_file" )
        .allowlist_function( "vips_image_new_from_buffer" )
        .allowlist_function( "vips_image_get_width" )
        .allowlist_function( "vips_image_get_height" )
        .allowlist_function( "vips_jpegsave" )
        .allowlist_function( "vips_pngsave" )
        .allowlist_function( "vips_webpsave" )
        .allowlist_function( "vips_jpegsave_buffer" )
        .allowlist_function( "vips_pngsave_buffer" )
        .allowlist_function( "vips_webpsave_buffer" )
        .allowlist_function( "vips_crop" )
        .allowlist_function( "vips_smartcrop" )
        .allowlist_function( "vips_array_double_new" )
        .allowlist_function( "vips_resize" )
        .allowlist_function( "vips_image_get_as_string" )
        .allowlist_function( "g_object_unref" )
        .allowlist_function( "g_free" )
        .allowlist_function( "vips_pdfload" )
        .allowlist_function( "vips_pdfload_buffer" )
        .allowlist_var( "VipsInterpretation_VIPS_INTERPRETATION_XYZ" )
        .allowlist_type( "_VipsImage" )
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from( "lib" );
    bindings
        .write_to_file(out_path.join("bindings.build.rs"))
        .expect("Couldn't write bindings!");
}