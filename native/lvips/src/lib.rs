#[macro_use]
extern crate rustler_codegen;
extern crate num_cpus;

use lazy_static::lazy_static;
use rustler::{Encoder, Env, Error, Term, Atom};
use std::env;
mod libvips;
use libvips::{VipsImage};
use libvips::save_options::{JpegSaveOptions, PngSaveOptions, SmartcropOptions, Interesting};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
        atom auto;
        atom none;
        atom png;
        atom jpg;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}


#[module = "Elxvips.ResizeOptions"]
#[derive(NifStruct, Debug)]
struct ResizeOptions<'a> {
    pub width: Term<'a>,
    pub height: Term<'a>,
    pub resize_type: Term<'a>,
}

#[module = "Elxvips.SaveOptions"]
#[derive(NifStruct, Debug)]
struct SaveOptions<'a> {
    quality: u8,
    strip: bool,
    path: &'a str,
    format: Term<'a>,
    compression: u8,
}

#[module = "Elxvips.ImageFile"]
#[derive(NifStruct, Debug)]
struct ImageFile<'a> {
    pub path: &'a str,
    pub resize: ResizeOptions<'a>,
    pub save: SaveOptions<'a>,
}

#[module = "Elxvips.ImageBytes"]
#[derive(NifStruct, Debug)]
struct ImageBytes<'a> {
    pub bytes: Vec<u8>,
    pub resize: ResizeOptions<'a>,
    pub save: SaveOptions<'a>,
}

rustler::rustler_export_nifs! {
    "Elixir.Elxvips",
    [
        ("vips_process_file_to_file", 1, process_file_to_file),
        ("vips_process_file_to_bytes", 1, process_file_to_bytes),
        ("vips_process_bytes_to_bytes", 1, process_bytes_to_bytes),
        ("vips_process_bytes_to_file", 1, process_bytes_to_file),
        ("vips_get_image_sizes", 1, get_image_sizes),
        ("vips_get_image_bytes_sizes", 1, get_image_bytes_sizes),
        ("vips_set_concurrency", 1, set_concurrency),
    ],
    Some(on_load)
}

lazy_static! {
    static ref JPEG_ATOM: Atom                      = atoms::jpg();
    static ref PNG_ATOM: Atom                       = atoms::png();
    static ref SMART_CROP_OPTS: SmartcropOptions    = SmartcropOptions {
        interesting: Interesting::Centre,
    };
}

fn image_into_bytes<'a>(image: VipsImage, save_options: SaveOptions) -> Result<Vec<u8>, String> {
    match save_options.format.decode::<Atom>() {
        Ok( format ) => {
            match format {
                format if format == atoms::jpg() => {
                    let options = JpegSaveOptions {
                        q: save_options.quality as i32,
                        strip: save_options.strip,
                        optimize_coding: true,
                        optimize_scans: true,
                        interlace: true,
                        ..JpegSaveOptions::default()
                    };

                    match image.jpeg_buffer_opts(&options) {
                        Ok ( bytes ) => Ok( bytes ),
                        Err( err )  => Err( format!( "failed to save image: {}", err ) )
                    }

                },
                format if format == atoms::png() => {
                    let options = PngSaveOptions {
                        q: save_options.quality as i32,
                        strip: save_options.strip,
                        compression: save_options.compression as i32,
                        interlace: true,
                        ..PngSaveOptions::default()
                    };

                    match image.png_buffer_opts(&options){
                        Ok ( bytes ) => {
                            Ok( bytes )
                        }
                        Err( err )  => Err( format!( "failed to save image: {}", err ) )
                    }

                },
                _ => Err( "format not supported".to_string() )
            }
        },
        Err( _ ) => Err( "format not supported".to_string() )
    }
}

fn image_from_bytes(buffer: &[u8]) -> Result<VipsImage, String> {
    match VipsImage::from_buffer(buffer) {
        Ok( image ) => Ok( image ),
        Err( err ) => Err( format!( "failed to create image from buffer: {}", err ) )
    }
}

fn get_image_bytes_sizes<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let result = match args[0].decode::<Vec<u8>>() {
        Ok( bytes ) => {
            match image_from_bytes( &bytes ) {
                Ok( image ) => Ok( [ image.get_width(), image.get_height() ] ),
                Err( err ) => Err( format!( "failed to read image from bytes: {}", err ) )
            }
        }
        Err( _ ) => Err( "failed to parse input data".to_string() )
    };
    match result {
        Ok( bytes ) => Ok( ( atoms::ok(), bytes.encode( env ) ).encode( env ) ),
        Err( error_str ) => Ok( ( atoms::error(), error_str ).encode( env ) )
    }
}

fn on_load(_env: Env, _info: Term) -> bool {
    let concurrency = match env::var( "VIPS_CONCURRENCY" ) {
        Ok( var ) => match var.parse::<u8>() {
            Ok( num ) => num,
            Err( _ ) => {
                panic!( "Couldn't convert VIPS_CONCURRENCY={:?} to int", var )
            }
        },
        Err(_) => num_cpus::get() as u8
    };
    libvips::concurrency_set(concurrency as i32);
    true
}

fn get_image_sizes<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let result = match args[0].decode::<String>() {
        Ok( image_path ) => {
            match VipsImage::from_file( &image_path ) {
                Ok( image ) => Ok( [ image.get_width(), image.get_height() ] ),
                Err( err ) => Err( format!( "failed to open image: {}", err ) )
            }
        },
        Err( _ ) => Err( "failed to parse input data".to_string() )
    };

    match result {
        Ok( image_sizes ) => Ok( ( atoms::ok(), image_sizes.encode( env ) ).encode( env ) ),
        Err( error_str ) => Ok( ( atoms::error(), error_str ).encode( env ) )
    }
}

fn set_concurrency<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let concurrency: u8 = args[0].decode()?;
    libvips::concurrency_set( concurrency as i32 );
    Ok( ( atoms::ok() ).encode( env ) )
}

fn resize_image<'a>(image: VipsImage, resize: &ResizeOptions<'a>) -> Result<VipsImage, String> {
    let source_width = image.get_width();
    let source_height = image.get_height();

    let target_width = match resize.width.decode::<i32>() {
        Ok( target_width ) => target_width,
        Err( _ ) => 0,
    };

    let target_height = match resize.height.decode::<i32>() {
        Ok( target_height ) => target_height,
        Err( _ ) => 0,
    };

    let original_size = ( target_width == 0 && target_height == 0 ) ||
        ( target_width == source_width && target_height == source_height ) ||
        ( target_width == source_width && target_height == 0 ) ||
        ( target_height == source_height && source_width == 0 );

    if original_size {
        Ok( image )
    } else {
        let source_ratio = source_width as f64 / source_height as f64;

        let target_width_f64 = ( target_height as f64 * source_width as f64 / source_height as f64 ) * ( target_width == 0 ) as i32 as f64 +
            target_width as f64 * ( target_width != 0 ) as i32 as f64;
        let target_height_f64 = ( target_width as f64 * source_height as f64 / source_width as f64 ) * ( target_height == 0 ) as i32 as f64 +
            target_height as f64 * ( target_height != 0 ) as i32 as f64;

        let target_ratio = target_width_f64 / target_height_f64;
        
        let resize_width = 
                source_width as f64 * target_height_f64 / source_height as f64  * ( source_ratio >= target_ratio ) as i32 as f64 +
                target_width_f64 * ( source_ratio < target_ratio ) as i32 as f64;

        let scale = resize_width.ceil() / source_width as f64;

        match image.resize( scale ) {
            Ok( resized ) => {

                match resized.smart_crop_opts(target_width_f64 as i32, target_height_f64 as i32, &SMART_CROP_OPTS) {
                    Ok( cropped ) => Ok( cropped ),
                    Err( err ) => Err( format!( "failed to crop image: {}", err ) )
                }
            },
            Err( err ) => Err( format!( "failed to resize image: {}", err ) )
        }

    }

}

fn save_image<'a>( image: &VipsImage, save_options: &SaveOptions<'a> ) -> Result<(), String> {
    match save_options.format.decode::<Atom>() {
        Ok( format ) => {
            match format {
                format if format == atoms::jpg() => {
                    let options = JpegSaveOptions {
                        q: save_options.quality as i32,
                        strip: save_options.strip,
                        optimize_coding: true,
                        optimize_scans: true,
                        interlace: true,
                        ..JpegSaveOptions::default()
                    };

                    match image.save_jpeg_opts(&save_options.path, &options) {
                        Ok ( () ) => Ok( () ),
                        Err( err )  => Err( format!( "failed to save image: {}", err ) )
                    }

                },
                format if format == atoms::png() => {
                    let options = PngSaveOptions {
                        q: save_options.quality as i32,
                        compression: save_options.compression as i32,
                        strip: save_options.strip,
                        interlace: true,
                        ..PngSaveOptions::default()
                    };

                    match image.save_png_opts(&save_options.path, &options) {
                        Ok ( () ) => Ok( () ),
                        Err( err )  => Err( format!( "failed to save image: {}", err ) )
                    }

                },
                _ => Err( "format not supported".to_string() )
            }
        },
        Err( _ ) => Err( "format not supported".to_string() )
    }
}

fn process_file_to_file<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let result = match args[0].decode::<ImageFile>() {
        Ok( image_input ) => {
            match VipsImage::from_file( &image_input.path ) {
                Ok( image ) => {
                    match resize_image( image, &image_input.resize ) {
                        Ok( image ) => save_image( &image, &image_input.save ),
                        Err( err ) => Err( err )
                    }
                },
                Err( err ) => Err( format!( "failed to open image: {}", err ) )
            }
        },
        Err( _ ) => Err( "failed to parse input data".to_string() )
    };

    match result {
        Ok( _ ) => Ok( ( atoms::ok() ).encode( env ) ),
        Err( err ) => Ok( ( atoms::error(), err ).encode( env ) )
    }
}

fn process_file_to_bytes<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let result = match args[0].decode::<ImageFile>() {
        Ok( image_input ) => {
            let save_options = image_input.save;
            let resize_options = image_input.resize;
            let path = image_input.path;
            match VipsImage::from_file( &path ) {
                Ok( image ) => {
                    match resize_image( image, &resize_options ) {
                        Ok( image ) => image_into_bytes( image, save_options ),
                        Err( err ) => Err( err )
                    }
                },
                Err( err ) => Err( format!( "failed to open image: {}", err ) )
            }
        },
        Err( _ ) => Err( "failed to parse input data".to_string() )
    };

    match result {
        Ok( bytes ) => Ok( ( atoms::ok(), bytes ).encode( env ) ),
        Err( err ) => Ok( ( atoms::error(), err ).encode( env ) )
    }
}

fn process_bytes_to_bytes<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let result = match args[0].decode::<ImageBytes>() {
        Ok( image_input ) => {
            let resize_options = image_input.resize;
            let save_options = image_input.save;
            match VipsImage::from_buffer( &image_input.bytes ) {
                Ok( image ) => {
                    match resize_image( image, &resize_options ) {
                        Ok( image ) => image_into_bytes( image, save_options ),
                        Err( err ) => Err( err )
                    }
                },
                Err( err ) => Err( format!( "failed to open image: {}", err ) )
            }
        },
        Err( _ ) => Err( "failed to parse input data".to_string() )
    };

    match result {
        Ok( bytes ) => Ok( ( atoms::ok(), bytes ).encode( env ) ),
        Err( err ) => Ok( ( atoms::error(), err ).encode( env ) )
    }
}

fn process_bytes_to_file<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let result = match args[0].decode::<ImageBytes>() {
        Ok( image_input ) => {
            let resize_options = image_input.resize;
            let save_options = image_input.save;
            match VipsImage::from_buffer( &image_input.bytes ) {
                Ok( image ) => {
                    match resize_image( image, &resize_options ) {
                        Ok( image ) => save_image( &image, &save_options ),
                        Err( err ) => Err( err )
                    }
                },
                Err( err ) => Err( format!( "failed to open image: {}", err ) )
            }
        },
        Err( _ ) => Err( "failed to parse input data".to_string() )
    };

    match result {
        Ok( () ) => Ok( ( atoms::ok() ).encode( env ) ),
        Err( err ) => Ok( ( atoms::error(), err ).encode( env ) )
    }
}