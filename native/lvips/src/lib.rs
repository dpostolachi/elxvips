extern crate num_cpus;

use rustler::{Encoder, Env, Error, Term, Atom, NifStruct, NifResult, Binary, OwnedBinary};
use std::env;
mod libvips;
use libvips::{VipsImage, VipsFormat};
use libvips::save_options::{JpegSaveOptions, PngSaveOptions, WebPSaveOptions, SmartcropOptions, Interesting, HeifsaveOptions};
use rustler::types::atom::{ok, error};

mod atoms {
    rustler::atoms! {
        auto,
        none,
        png,
        jpg,
        webp,
        avif,
        svg,
    }
}

#[derive(NifStruct, Debug)]
#[module = "Elxvips.ResizeOptions"]
struct ResizeOptions {
    pub width: i32,
    pub height: i32,
    pub resize_type: Atom,
}

#[derive(NifStruct, Debug)]
#[module = "Elxvips.SaveOptions"]
struct SaveOptions {
    quality: u8,
    strip: bool,
    path: String,
    format: Atom,
    compression: u8,
    background: Vec<f64>,
}

#[derive(NifStruct, Debug)]
#[module = "Elxvips.ImageFile"]
struct ImageFile {
    pub path: String,
    pub resize: ResizeOptions,
    pub save: SaveOptions,
    pub pdf: bool,
    pub page: i32,
    pub n: i32,
}

#[derive(NifStruct)]
#[module = "Elxvips.ImageBytes"]
struct ImageBytes<'a> {
    pub bytes: Binary<'a>,
    pub resize: ResizeOptions,
    pub save: SaveOptions,
    pub pdf: bool,
    pub page: i32,
    pub n: i32,
}

static SMART_CROP_OPTS: SmartcropOptions = SmartcropOptions {
    interesting: Interesting::Centre,
};

fn format_to_atom( format: VipsFormat ) -> Atom {
    match format {
        VipsFormat::JPEG => atoms::jpg(),
        VipsFormat::PNG => atoms::png(),
        VipsFormat::WEBP => atoms::webp(),
        VipsFormat::AVIF => atoms::avif(),
        VipsFormat::SVG => atoms::svg(),
    }
}

fn image_into_bytes<'a>(image: VipsImage, save_options: &SaveOptions) -> Result<Vec<u8>, String> {

    let vips_format = match save_options.format {
        format if format == atoms::jpg() => VipsFormat::JPEG,
        format if format == atoms::png() => VipsFormat::PNG,
        format if format == atoms::webp() => VipsFormat::WEBP,
        format if format == atoms::avif() => VipsFormat::AVIF,
        format if format == atoms::svg() => VipsFormat::SVG,
        format if format == atoms::auto() => image.get_format().unwrap(),
        _ => {
            return Err( "format not supported".to_string() )
        }
    };

    match vips_format {
        VipsFormat::JPEG => {
            let options = JpegSaveOptions {
                q: save_options.quality as i32,
                strip: save_options.strip,
                optimize_coding: true,
                optimize_scans: true,
                interlace: true,
                background: save_options.background.to_owned(),
                ..JpegSaveOptions::default()
            };

            match image.jpeg_buffer_opts(&options) {
                Ok ( bytes ) => Ok( bytes ),
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }

        },
        VipsFormat::PNG => {
            let options = PngSaveOptions {
                q: save_options.quality as i32,
                strip: save_options.strip,
                compression: save_options.compression as i32,
                interlace: true,
                background: save_options.background.to_owned(),
                ..PngSaveOptions::default()
            };

            match image.png_buffer_opts(&options){
                Ok ( bytes ) => {
                    Ok( bytes )
                }
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }

        },
        VipsFormat::WEBP => {
            let options = WebPSaveOptions {
                q: save_options.quality as i32,
                strip: save_options.strip,
                background: save_options.background.to_owned(),
                ..WebPSaveOptions::default()
            };

            match image.webp_buffer_opts(&options) {
                Ok ( bytes ) => {
                    Ok( bytes )
                }
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }

        },
        VipsFormat::AVIF => {
            let options = HeifsaveOptions {
                q: save_options.quality as i32,
                background: save_options.background.to_owned(),
                compression: libvips::save_options::ForeignHeifCompression::Av1,
                ..HeifsaveOptions::default()
            };

            match image.avif_buffer_opts(&options) {
                Ok ( bytes ) => {
                    Ok( bytes )
                }
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }

        },
        VipsFormat::SVG => {
            match image.svg_buffer() {
                Ok ( bytes ) => {
                    Ok( bytes )
                }
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }
        }
    }
}

fn image_from_bytes(buffer: &[u8]) -> Result<VipsImage, String> {
    match VipsImage::from_buffer(buffer) {
        Ok( image ) => Ok( image ),
        Err( err ) => Err( format!( "failed to create image from buffer: {}", err ) )
    }
}

#[rustler::nif]
fn vips_get_image_bytes_sizes<'a>(env: Env<'a>, bytes: Binary<'a>) -> Result<Term<'a>, Error> {
    let result = match image_from_bytes( &bytes ) {
        Ok( image ) => Ok( [ image.get_width(), image.get_height() ] ),
        Err( err ) => Err( format!( "failed to read image from bytes: {}", err ) )
    };

    match result {
        Ok( bytes ) => Ok( ( ok(), bytes.encode( env ) ).encode( env ) ),
        Err( error_str ) => Ok( ( error(), error_str ).encode( env ) )
    }
}

#[rustler::nif]
fn vips_get_image_file_format<'a>(env: Env<'a>, path: &str) -> Result<Term<'a>, Error> {
    let result = match VipsImage::from_file( &path ) {
        Ok( image ) => {
            Ok( image.get_format().unwrap() )
        },
        Err( err ) => Err( format!( "failed to open image: {}", err ) )
    };

    match result {
        Ok( format ) => Ok( ( ok(), format_to_atom( format ) ).encode( env ) ),
        Err( err ) => Ok( ( error(), err ).encode( env ) )
    }
}

#[rustler::nif]
fn vips_get_image_bytes_format<'a>(env: Env<'a>, bytes: Binary<'a>) -> Result<Term<'a>, Error> {
    let result = match image_from_bytes( &bytes ) {
        Ok( image ) => {
            Ok( image.get_format().unwrap() )
        },
        Err( err ) => Err( format!( "failed to read image from bytes: {}", err ) )
    };
    match result {
        Ok( format ) => Ok( ( ok(), format_to_atom( format ) ).encode( env ) ),
        Err( error_str ) => Ok( ( error(), error_str ).encode( env ) )
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

#[rustler::nif]
fn vips_get_image_sizes<'a>(env: Env<'a>, image_path: &str ) -> Result<Term<'a>, Error> {
    let result = match VipsImage::from_file( &image_path ) {
        Ok( image ) => Ok( [ image.get_width(), image.get_height() ] ),
        Err( err ) => Err( format!( "failed to open image: {}", err ) )
    };

    match result {
        Ok( image_sizes ) => Ok( ( ok(), image_sizes.encode( env ) ).encode( env ) ),
        Err( error_str ) => Ok( ( error(), error_str ).encode( env ) )
    }
}

#[rustler::nif]
fn set_concurrency<'a>(env: Env<'a>, concurrency: u8) -> Result<Term<'a>, Error> {
    libvips::concurrency_set( concurrency as i32 );
    Ok( ( ok() ).encode( env ) )
}

fn resize_image(image: VipsImage, resize: &ResizeOptions) -> Result<VipsImage, String> {
    let source_width = image.get_width();
    let source_height = image.get_height();

    let target_width = resize.width;
    let target_height = resize.height;

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

fn save_image( image: &VipsImage, save_options: &SaveOptions ) -> Result<(), String> {

    let vips_format = match save_options.format {
        format if format == atoms::jpg() => VipsFormat::JPEG,
        format if format == atoms::png() => VipsFormat::PNG,
        format if format == atoms::webp() => VipsFormat::WEBP,
        format if format == atoms::avif() => VipsFormat::AVIF,
        format if format == atoms::svg() => VipsFormat::SVG,
        format if format == atoms::auto() => image.get_format().unwrap(),
        _ => {
            return Err( "format not supported".to_string() )
        }
    };

    match vips_format {
        VipsFormat::JPEG => {
            let options = JpegSaveOptions {
                q: save_options.quality as i32,
                strip: save_options.strip,
                optimize_coding: true,
                optimize_scans: true,
                interlace: true,
                background: save_options.background.to_owned(),
                ..JpegSaveOptions::default()
            };

            match image.save_jpeg_opts(&save_options.path, &options) {
                Ok ( () ) => Ok( () ),
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }

        },
        VipsFormat::PNG => {
            let options = PngSaveOptions {
                q: save_options.quality as i32,
                compression: save_options.compression as i32,
                strip: save_options.strip,
                interlace: true,
                background: save_options.background.to_owned(),
                ..PngSaveOptions::default()
            };

            match image.save_png_opts(&save_options.path, &options) {
                Ok ( () ) => Ok( () ),
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }

        },
        VipsFormat::WEBP => {
            let options = WebPSaveOptions {
                q: save_options.quality as i32,
                strip: save_options.strip,
                background: save_options.background.to_owned(),
                ..WebPSaveOptions::default()
            };

            match image.save_webp_opts(&save_options.path, &options) {
                Ok ( () ) => Ok( () ),
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }

        },
        VipsFormat::AVIF => {
            let options = HeifsaveOptions {
                q: save_options.quality as i32,
                background: save_options.background.to_owned(),
                compression: libvips::save_options::ForeignHeifCompression::Av1,
                ..HeifsaveOptions::default()
            };

            match image.save_heif_opts(&save_options.path, &options) {
                Ok ( () ) => Ok( () ),
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }
        },
        VipsFormat::SVG => {
            match image.save_svg(&save_options.path) {
                Ok ( () ) => Ok( () ),
                Err( err )  => Err( format!( "failed to save image: {}", err ) )
            }
        }
    }
}

fn load_from_file( image_input: &ImageFile ) -> Result<VipsImage, String> {
    match &image_input.pdf {
        true => VipsImage::from_pdf_file( &image_input.path, &image_input.page, &image_input.n ),
        false => VipsImage::from_file( &image_input.path )
    }
}

fn load_from_buffer( image_input: &ImageBytes ) -> Result<VipsImage, String> {
    match &image_input.pdf {
        true => VipsImage::from_pdf_buffer( &image_input.bytes, &image_input.page, &image_input.n ),
        false => VipsImage::from_buffer( &image_input.bytes )
    }
}

#[rustler::nif]
fn vips_process_file_to_file(image_input: ImageFile) -> NifResult<Atom> {
    let result = match load_from_file( &image_input ) {
        Ok( image ) => {
            match resize_image( image, &image_input.resize ) {
                Ok( image ) => save_image( &image, &image_input.save ),
                Err( err ) => Err( err )
            }
        },
        Err( err ) => Err( format!( "failed to open image: {}", err ) )
    };

    match result {
        Ok( () ) => Ok( ok() ),
        Err( err ) => Err( Error::Term( Box::new( err ) ) )
    }
}

#[rustler::nif]
fn vips_process_file_to_bytes<'a>(env: Env<'a>, image_input: ImageFile) -> Result<Term<'a>, Error> {
    let result = match load_from_file( &image_input ) {
        Ok( image ) => {
            match resize_image( image, &image_input.resize ) {
                Ok( image ) => image_into_bytes( image, &image_input.save ),
                Err( err ) => Err( err )
            }
        },
        Err( err ) => Err( format!( "failed to open image: {}", err ) )
    };

    match result {
        Ok( bytes ) => {
            let mut own_binary = OwnedBinary::new( bytes.len() ).unwrap();
            own_binary.as_mut_slice().copy_from_slice( &bytes );
            let binary = Binary::from_owned( own_binary, env );
            Ok( ( ok(), binary ).encode( env ) )
        },
        Err( err ) => Ok( ( error(), err ).encode( env ) )
    }
}

#[rustler::nif]
fn vips_process_bytes_to_bytes<'a>(env: Env<'a>, image_input: ImageBytes) -> Result<Term<'a>, Error> {
    let result = match load_from_buffer( &image_input ) {
        Ok( image ) => {
            match resize_image( image, &image_input.resize ) {
                Ok( image ) => image_into_bytes( image, &image_input.save ),
                Err( err ) => Err( err )
            }
        },
        Err( err ) => Err( format!( "failed to open image: {}", err ) )
    };

    match result {
        Ok( bytes ) => {
            let mut binary = OwnedBinary::new( bytes.len() ).unwrap();
            binary.as_mut_slice().copy_from_slice( &bytes );
            let binary = Binary::from_owned( binary, env );
            Ok( ( ok(), binary ).encode( env ) )
        },
        Err( err ) => Ok( ( error(), err ).encode( env ) )
    }
}

#[rustler::nif]
fn vips_process_bytes_to_file<'a>(env: Env<'a>, image_input: ImageBytes) -> Result<Term<'a>, Error> {
    let result = match load_from_buffer( &image_input ) {
        Ok( image ) => {
            match resize_image( image, &image_input.resize ) {
                Ok( image ) => save_image( &image, &image_input.save ),
                Err( err ) => Err( err )
            }
        },
        Err( err ) => Err( format!( "failed to open image: {}", err ) )
    };

    match result {
        Ok( () ) => Ok( ( ok() ).encode( env ) ),
        Err( err ) => Ok( ( error(), err ).encode( env ) )
    }
}

rustler::init!("Elixir.Elxvips.Native", load=on_load );
