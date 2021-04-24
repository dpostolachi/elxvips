extern crate bindgen;
use std::process::Command;
use std::path::{ PathBuf };
use std::fs;
use std::env;
use flate2::read::GzDecoder;
use tar::Archive;
use num_cpus;

static VIPS_TAR: &'static str = "https://github.com/libvips/libvips/releases/download/v8.10.6/vips-8.10.6.tar.gz";

fn check_installed_vips() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("pkg-config --cflags vips")
        .output()
        .unwrap()
        .status
        .success()
}

fn main() {

    let ( vips_include_path, vips_lib_path ) = if !check_installed_vips() {

        let cpus =                  num_cpus::get() as u8; // used for -j option for make

        let mut dist_path =         PathBuf::from( env::current_dir().unwrap() );
            dist_path.push( "../common" );
        let dist =                  dist_path.as_path().to_str().unwrap();
    
        let vips_include_path =     format!( "-I{}/include", dist );
        let vips_lib_path =         format!( "-L{}/lib", dist );
    
        // download and extract vips tar
        let tar_gz =                reqwest::blocking::get( VIPS_TAR ).unwrap()
            .bytes().unwrap();
    
        let tar =                   GzDecoder::new(&*tar_gz);
        let mut archive =           Archive::new(tar);
    
        archive.unpack("./lib").unwrap();

        // configure
        Command::new( "sh" )
            .current_dir( "./lib/vips-8.10.6" )
            .arg( "-c" )
            .arg( "./configure" )
            .output()
            .unwrap()
            .stdout;

        // make
        Command::new( "sh" )
            .current_dir( "./lib/vips-8.10.6" )
            .arg( "-c" )
            .arg(
                format!( "make -j{}", &cpus.to_string() )
            )
            .output()
            .unwrap()
            .stdout;

        // make install
        Command::new( "sh" )
            .current_dir( "./lib/vips-8.10.6" )
            .arg( "-c" )
            .arg(
                format!(
                    "make install -j{} prefix={}",
                    &cpus.to_string(), dist
                )
            )
            .output()
            .unwrap()
            .stdout;

        fs::remove_dir_all("lib/vips-8.10.6")
            .unwrap();

        ( vips_include_path, vips_lib_path )
    

    } else {
        // link existing vips
        println!("cargo:rustc-link-lib=vips");

        ( "".to_string(), "".to_string() )
    };

    // search for glib location
    let pkg_config_out = Command::new("sh")
        .arg("-c")
        .arg("pkg-config --cflags glib-2.0")
        .output()
        .unwrap()
        .stdout;

    let out_str = String::from_utf8_lossy( &pkg_config_out );
    let out_paths: Vec<&str> = out_str.split( ' ' )
        .collect();
    let ( glib2_path, glib2_conf_path ) = (
        out_paths[0].trim(),
        out_paths[1].trim(),
    );

    // link glib2
    println!("cargo:rustc-link-lib=glib-2.0");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=lib/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("lib/wrapper.h")
        .clang_arg( glib2_path )
        .clang_arg( glib2_conf_path )
        .clang_arg( vips_include_path )
        .clang_arg( vips_lib_path )
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

    let out_path = PathBuf::from( "../common" );
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
}