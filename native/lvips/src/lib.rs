extern crate rustler;
#[macro_use]
extern crate rustler_codegen;

use rustler::{Encoder, Env, Error, Term, Atom};
use libvips::{ops, VipsImage, VipsApp};
use libvips::{ops::{SmartcropOptions, Interesting}};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
        atom auto;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}


#[module = "ResizeOptions"]
#[derive(NifStruct, Debug)]
struct ResizeOptions<'a> {
    pub width: Term<'a>,
    pub height: Term<'a>,
    pub fill: Term<'a>,
}

#[module = "SaveOptions"]
#[derive(NifStruct, Debug)]
struct SaveOptions<'a> {
    quality: u8,
    strip: bool,
    path: String,
    format: Term<'a>,
}

#[module = "ImageFile"]
#[derive(NifStruct, Debug)]
struct ImageFile<'a> {
    pub path: String,
    pub resize: ResizeOptions<'a>,
    pub save: SaveOptions<'a>,
}


rustler::rustler_export_nifs! {
    "Elixir.Elxvips",
    [
        ("process_image", 1, process_image)
    ],
    None
}

fn process_image<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let image_input: ImageFile = args[0].decode()?;
    println!( ">>>{:?}", image_input);
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");

    app.concurrency_set(2);

    match VipsImage::new_from_file( &image_input.path ) {
        Ok( image ) => {

            let width = image.get_width();
            let height = image.get_height();

            let target_width_i32 = match image_input.resize.width.decode::<i32>() {
                Ok( target_width ) => target_width,
                Err( _ ) => 0,
            };

            let target_height_i32 = match image_input.resize.height.decode::<i32>() {
                Ok( target_height ) => target_height,
                Err( _ ) => 0,
            };

            // let target_scale = target_width_i32 / target_height_i32;
            let max_size = width * ( width > height ) as i32 + height * ( height >= width ) as i32;
            let min_size = width * ( max_size == height ) as i32 + height * ( max_size == width ) as i32 + ( -max_size * ( height == width ) as i32 ); // TODO: cover rects

            let max_target = target_width_i32 * ( target_width_i32 > target_height_i32 ) as i32 +
                target_height_i32 * ( target_width_i32 < target_height_i32 ) as i32 +
                max_size * ( target_width_i32 == target_height_i32 ) as i32;

            // let scale = max_target as f64 / max_size as f64;
            let scale = ( max_target as f64 / width as f64 ) * ( max_target == target_width_i32 ) as i32 as f64 +
                ( max_target as f64 / height as f64 ) * ( max_target == target_height_i32 ) as i32 as f64;

            let min_target = target_width_i32 as f64 * ( max_target == target_height_i32 ) as i32 as f64 +
                target_height_i32 as f64 * ( max_target == target_width_i32 ) as i32 as f64 +
                ( scale * min_size as f64 ) as f64 * ( target_width_i32 == 0 || target_height_i32 == 0 ) as i32 as f64;

            let ( crop_width, crop_height ) = (
                max_target as f64 * ( max_target == target_width_i32 ) as i32 as f64 + min_target * ( max_target != target_width_i32 ) as i32 as f64,
                max_target as f64 * ( max_target == target_height_i32 ) as i32 as f64 + min_target * ( max_target != target_height_i32 ) as i32 as f64
            );

            match ops::resize( &image, scale ) {

                Ok( resized ) => {

                    let options: SmartcropOptions = SmartcropOptions{
                        interesting: Interesting::Centre,
                    };

                    match ops::smartcrop_with_opts( &resized, crop_width as i32, crop_height as i32, &options ) {
                        Ok( cropped ) => {

                            match image_input.save.format.decode::<Atom>() {

                                Ok( format ) => {

                                    let jpg_atom = Atom::from_str( env, "jpg2" )?;

                                    if format == jpg_atom {
     
                                        Ok( ( atoms::error(), "failed to resize image" ).encode( env ) )
     
                                    } else {

                                        let options = ops::JpegsaveOptions {
                                            q: image_input.save.quality as i32,
                                            strip: image_input.save.strip,
                                            optimize_coding: true,
                                            optimize_scans: true,
                                            interlace: true,
                                            ..ops::JpegsaveOptions::default()
                                        };

                                        match ops::jpegsave_with_opts(&cropped, &image_input.save.path, &options) {
                                            Ok ( _ ) => Ok( ( atoms::ok() ).encode( env ) ),
                                            Err( _ )  => Ok( ( atoms::error(), "Failed to save image" ).encode( env ) )
                                        }

                                    }

                                },

                                Err( _ ) => Ok( ( atoms::error(), "format not supported" ).encode( env ) )

                            }

                        },

                        Err( _ ) => {
                            println!("error: {}", app.error_buffer().unwrap());
                            Ok( ( atoms::error(), "failed to crop image" ).encode( env ) )
                        }
                    }

                },
                Err( _ ) => Ok( ( atoms::error(), "failed to resize image" ).encode( env ) )
            }


            // if target_width_u32 < 1 {
            //     target_width_u32 = match image_input.resize.width.decode::<&str>() {
            //         Ok(&_) => width,
            //         Err( _ ) => width,
            //     };
            // }

            // let target_width = match image_input.resize.width.decode::<&'static str>()? {
            //         // Term.Atom{} => 123,
            //         "auto" => width,
            // };

            // let scale = width / 
            // let (  )
            // if max_size === width {
            //     let scale = 
            // } else {

            // }

            // Ok( ( atoms::ok(), target_width_i32 ).encode( env ) )
        },
        Err(_) => {
            Ok( ( atoms::error(), "failed to open image" ).encode( env ) )
        }
    }

    // let image = VipsImage::new_from_file( &image_input.path ).unwrap();

    // println!( ">>>{:?}", input);
    // println!( ">>> called" );
    // let output: String = args[1].decode()?;

    // match resize_img( &input, &output ) {
    //     Ok(_) => Ok( atoms::ok().encode(env) ),
    //     Err(err) => Ok( ( atoms::error(), err ).encode(env) )
    // }
    // Ok( ( atoms::ok() ).encode( env ) )
}