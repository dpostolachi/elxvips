extern crate bindgen;
use std::process::Command;
use std::path::PathBuf;
use std::fs;
use flate2::read::GzDecoder;
use tar::Archive;
use num_cpus;

fn main() {

    // let tar_gz = reqwest::blocking::get("https://github.com/libvips/libvips/releases/download/v8.10.6/vips-8.10.6.tar.gz").unwrap()
    //     .bytes().unwrap();

    // let tar = GzDecoder::new(&*tar_gz);
    // let mut archive = Archive::new(tar);
    // archive.unpack("./lib").unwrap();

    // let cpus = num_cpus::get() as u8;
    // let parallel_par = String::from( " -j" ) + &cpus.to_string();
    // println!( "{:?}", parallel_par.to_string() );


    // Command::new( "sh" )
    //     .current_dir( "./lib/vips-8.10.6" )
    //     .arg( "-c" )
    //     .arg( "./configure" )
    //     .output()
    //     .unwrap()
    //     .stdout;
    

    // let make_out = Command::new( "sh" )
    //     .current_dir( "./lib/vips-8.10.6" )
    //     .arg( "-c" )
    //     .arg( "make -j16" )
    //     .output()
    //     .unwrap()
    //     .stdout;

    // println!( "{}", &String::from_utf8_lossy( &make_out ) );

    let pwd_out = &Command::new( "sh" )
        .arg( "-c" )
        .arg( "pwd" )
        .output()
        .unwrap()
        .stdout;

    let dirty_out_path = String::from_utf8_lossy( &pwd_out );
    let pwd_path = String::from( dirty_out_path.trim() );

    let lib_path = pwd_path + "/lib/vips-8.10.6/tmp";

    // let make_install_arg = String::from( "make install -j16 prefix=" );

    // let make_install_out = Command::new( "sh" )
    //     .current_dir( "./lib/vips-8.10.6" )
    //     .arg( "-c" )
    //     .arg( make_install_arg + &lib_path )
    //     .output()
    //     .unwrap()
    //     .stdout;

    // println!( "{}", &String::from_utf8_lossy( &make_install_out ) );

    let pkg_config_out = Command::new("sh")
        .arg("-c")
        .arg("pkg-config --cflags glib-2.0")
        .output()
        .unwrap()
        .stdout;

    let out_str = String::from_utf8_lossy( &pkg_config_out );
    println!("{}", &out_str);
    let out_paths: Vec<&str> = out_str.split( ' ' )
        .collect();
    let ( glib2_path, glib2_conf_path ) = (
        out_paths[0].trim(),
        out_paths[1].trim(),
    );


    let mut vips_include_1 = String::from( "-I" );
    vips_include_1 = vips_include_1 + &lib_path + "/include";

    let mut vips_lib_1 = String::from( "-L" );
    vips_lib_1 = vips_lib_1 + &lib_path + "/lib";

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-search=/home/dumitras/Projects/elixir/elxvips/native/lvips/lib/vips-8.10.6/tmp/lib");
    // println!("cargo:cargo:rustc-link-search=lib/vips-8.10.6/tmp/lib");
    println!("cargo:rustc-link-lib=static=vips");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=lib/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("lib/vips-8.10.6/tmp/include/vips/vips.h")
        .clang_arg( glib2_path )
        .clang_arg( glib2_conf_path )
        .clang_arg( vips_include_1 )
        .clang_arg( vips_lib_1 )
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
    
    // fs::remove_dir_all("lib/vips-8.10.6")
    //     .unwrap();
}