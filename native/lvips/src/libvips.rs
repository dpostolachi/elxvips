#![allow(unused)]
pub mod bindings;
pub mod utils;
pub mod save_options;
pub mod globals;
use std::ffi::{CStr, c_void};
use utils::{c_string, null};
use std::ffi::{CString};
use save_options::{JpegSaveOptions, PngSaveOptions, WebPSaveOptions, SmartcropOptions};
use base64::{engine::general_purpose, Engine as _};
use std::fs;

use self::save_options::HeifsaveOptions;

#[derive(PartialEq)]
pub enum VipsFormat {
    PNG,
    JPEG,
    WEBP,
    AVIF,
    SVG,
}

pub fn error_buffer() -> String {
    unsafe {
        let error = CStr::from_ptr( bindings::vips_error_buffer() )
            .to_str()
            .unwrap();
        let out = String::from( error );
        bindings::vips_error_clear();
        out
    }
}

pub fn concurrency_set( max: i32 ) {
    unsafe {
        bindings::vips_concurrency_set(max);
    }
}

pub fn vips_init() {
    unsafe {
        bindings::vips_init( CString::new( "lvips" ).unwrap().as_ptr() );
    }
}

#[derive(Clone)]
enum ImageSource {
    File( String ),
    Buffer( Vec<u8> ),
    None,
}

pub struct VipsImage {
    image: *mut bindings::_VipsImage,
    source: ImageSource,
}

impl VipsImage {
    pub fn get_width( &self ) -> i32 {
        unsafe {
            bindings::vips_image_get_width( self.image )
        }
    }
    pub fn destroy( &self ) {
        unsafe{
            bindings::g_object_unref( self.image as *mut c_void );
        }
    }
    pub fn get_height( &self ) -> i32 {
        unsafe {
            bindings::vips_image_get_height( self.image )
        }
    }
    pub fn get_string( &self, vips_string: &CStr ) -> Result<String, String> {
        unsafe {
            let params = globals::get_params().unwrap();
            let mut out = null();

            match bindings::vips_image_get_as_string( self.image, vips_string.as_ptr(), &mut out ) {
                0 => {
                    // Note: string from bindings must be owned and received copy must
                    // be freed
                    let string = CStr::from_ptr( out ).to_str().unwrap().to_string();
                    bindings::g_free( out as *mut c_void );
                    Ok( string )
                },
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn get_format( &self ) -> Result<VipsFormat, String> {
        let params = globals::get_params().unwrap();
        let format_string: &str = &self.get_string( &params.vips_loader ).unwrap();
        match format_string {
            "jpegload"  | "jpegload_buffer" => Ok( VipsFormat::JPEG ),
            "pngload"   | "pngload_buffer" => Ok( VipsFormat::PNG ),
            "webpload"   | "webpload_buffer" => Ok( VipsFormat::WEBP ),
            "heifload"  | "heifload_buffer" => Ok( VipsFormat::AVIF ),
            "svgload"   | "svgload_buffer" => Ok( VipsFormat::SVG ),
            _ => Err( "unknown format".to_string() )
        }
    }
    pub fn from_file( path: &str ) -> Result<VipsImage, String> {
        let filename = c_string( path ).unwrap();
        unsafe {

            let image = bindings::vips_image_new_from_file( filename.as_ptr(), utils::NULL );

            if image.is_null() {
                Err( error_buffer() )
            } else {
                Ok( VipsImage{
                    image: image,
                    source: ImageSource::File( path.to_string() ),
                } )
            }
        }
    }

    pub fn from_pdf_file( path: &str, page: &i32, n: &i32 ) -> Result<VipsImage, String> {
        let filename = c_string( path ).unwrap();
        let params = globals::get_params().unwrap();

        unsafe {

            let mut output: *mut bindings::VipsImage = null();

            match  bindings::vips_pdfload(
                filename.as_ptr(),
                &mut output,
                params.page.as_ptr(),         page.to_owned(),
                params.n.as_ptr(),            n.to_owned(),
                utils::NULL
            ) {
                0 => Ok( VipsImage{
                    image: output,
                    source: ImageSource::File( path.to_string() ),
                } ),
                _ => Err( error_buffer() )
            }

        }
    }

    pub fn from_pdf_buffer( buffer: &[u8], page: &i32, n: &i32 ) -> Result<VipsImage, String> {
        let params = globals::get_params().unwrap();
        let options = c_string("").unwrap();

        unsafe {
            let image = bindings::vips_image_new_from_buffer(
                buffer.as_ptr() as *const c_void,
                buffer.len() as usize,
                options.as_ptr(),
                params.page.as_ptr(),           page.to_owned(),
                params.n.as_ptr(),              n.to_owned(),
                utils::NULL
            );

            if image.is_null() {
                Err( error_buffer() )
            } else {
                Ok( VipsImage{
                    image: image,
                    source: ImageSource::Buffer( buffer.to_vec() ),
                } )
            }
        }
    }

    pub fn from_buffer( buffer: &[u8] ) -> Result<VipsImage, String> {
        let options = c_string("").unwrap();
        unsafe {
            let image = bindings::vips_image_new_from_buffer(
                buffer.as_ptr() as *const c_void,
                buffer.len() as usize,
                options.as_ptr(),
                utils::NULL
            );

            if image.is_null() {
                Err( error_buffer() )
            } else {
                Ok( VipsImage{
                    image: image,
                    source: ImageSource::Buffer( buffer.to_vec() ),
                } )
            }
        }

    }
    pub fn crop( &self, left: i32, top: i32, width: i32, height: i32 ) -> Result<VipsImage, String> {
        let input: *mut bindings::VipsImage = self.image;
        let mut output: *mut bindings::VipsImage = null();
        unsafe {
            match bindings::vips_crop( input, &mut output, left, top, width, height, utils::NULL ) {
                0 => Ok( VipsImage{
                    image: output,
                    source: self.source.clone(),
                } ),
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn smart_crop( &self, width: i32, height: i32 ) -> Result<VipsImage, String> {
        let input: *mut bindings::VipsImage = self.image;
        let mut output: *mut bindings::VipsImage = null();
        unsafe {
            match bindings::vips_smartcrop( input, &mut output, width, height, utils::NULL ) {
                0 => Ok( VipsImage{
                    image: output,
                    source: self.source.clone(),
                } ),
                _ => Err( error_buffer() )
            }
        } 
    }

    pub fn smart_crop_opts( &self, width: i32, height: i32, options: &SmartcropOptions ) -> Result<VipsImage, String> {
        let input: *mut bindings::VipsImage = self.image;
        let mut output: *mut bindings::VipsImage = null();
        let params = globals::get_params().unwrap();
        unsafe {
            match bindings::vips_smartcrop(
                input,
                &mut output,
                width,
                height,
                params.interesting.as_ptr(),       options.interesting as i32,
                utils::NULL
            ) {
                0 => Ok( VipsImage{
                    image: output,
                    source: self.source.clone(),
                } ),
                _ => Err( error_buffer() )
            }
        }  
    }

    pub fn save_png( &self, path: &str ) -> Result<(), String> {
        let filename = c_string( path ).unwrap();
        unsafe {
            match bindings::vips_pngsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn save_png_opts( &self, path: &str, options: &save_options::PngSaveOptions ) -> Result<(), String> {
        let filename = c_string( path ).unwrap();
        let profile = c_string(&options.profile).unwrap();
        let params = globals::get_params().unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);
            
            match bindings::vips_pngsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                params.compression.as_ptr(),        options.compression,
                params.interlace.as_ptr(),          options.interlace as i32,
                params.page_height.as_ptr(),        options.page_height,
                params.profile.as_ptr(),            profile.as_ptr(),
                params.filter.as_ptr(),             options.filter,
                params.palette.as_ptr(),            options.palette as i32,
                params.colours.as_ptr(),            options.colours,
                params.q.as_ptr(),                  options.q,
                params.dither.as_ptr(),             options.dither,
                params.strip.as_ptr(),              options.strip as i32,
                params.background.as_ptr(),         background_array,
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn save_webp( &self, path: &str ) -> Result<(), String> {
        let filename = c_string( path ).unwrap();
        unsafe {
            match bindings::vips_webpsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn save_webp_opts( &self, path: &str, options: &save_options::WebPSaveOptions ) -> Result<(), String> {
        let filename = c_string( path ).unwrap();
        let profile = c_string(&options.profile).unwrap();
        let params = globals::get_params().unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_webpsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                params.page_height.as_ptr(),        options.page_height,
                params.q.as_ptr(),                  options.q,
                params.strip.as_ptr(),              options.strip as i32,
                params.background.as_ptr(),         background_array,
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn save_heif_opts( &self, path: &str, options: &save_options::HeifsaveOptions ) -> Result<(), String> {
        let filename = c_string( path ).unwrap();
        let profile = c_string(&options.profile).unwrap();
        let params = globals::get_params().unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_heifsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                params.page_height.as_ptr(),        options.page_height,
                params.q.as_ptr(),                  options.q,
                params.background.as_ptr(),         background_array,
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn save_svg( &self, path: &str ) -> Result<(), String> {
        let buffer = self.svg_buffer().unwrap();
        fs::write( path, buffer ).unwrap();
        Ok( () )
    }

    pub fn save_jpeg( &self, path: &str ) -> Result<(), String> {
        let filename = c_string( path ).unwrap();

        unsafe {
            match bindings::vips_jpegsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn save_jpeg_opts( &self, path: &str, options: &JpegSaveOptions ) -> Result<(), String> {
        let filename = c_string( path ).unwrap();
        let profile = c_string(&options.profile).unwrap();
        let params = globals::get_params().unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_jpegsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                params.page_height.as_ptr(),            options.page_height,
                params.q.as_ptr(),                      options.q,
                params.profile.as_ptr(),                profile.as_ptr(),
                params.optimize_coding.as_ptr(),        options.optimize_coding as i32,
                params.interlace.as_ptr(),              options.interlace as i32,
                params.no_sub_sample.as_ptr(),          options.no_subsample as i32,
                params.trellis_quant.as_ptr(),          options.trellis_quant as i32,
                params.overshoot_deringing.as_ptr(),    options.overshoot_deringing as i32,
                params.optimize_scans.as_ptr(),         options.optimize_scans as i32,
                params.quant_table.as_ptr(),            options.quant_table,
                params.strip.as_ptr(),                  options.strip as i32,
                params.background.as_ptr(),             background_array,
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn jpeg_buffer( &self ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();

        unsafe {
            match bindings::vips_jpegsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn jpeg_buffer_opts( &self, options: &JpegSaveOptions ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();
        let profile = c_string(&options.profile).unwrap();
        let params = globals::get_params().unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_jpegsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                params.page_height.as_ptr(),            options.page_height,
                params.q.as_ptr(),                      options.q,
                params.profile.as_ptr(),                profile.as_ptr(),
                params.optimize_coding.as_ptr(),        options.optimize_coding as i32,
                params.interlace.as_ptr(),              options.interlace as i32,
                params.no_sub_sample.as_ptr(),          options.no_subsample as i32,
                params.trellis_quant.as_ptr(),          options.trellis_quant as i32,
                params.overshoot_deringing.as_ptr(),    options.overshoot_deringing as i32,
                params.optimize_scans.as_ptr(),         options.optimize_scans as i32,
                params.quant_table.as_ptr(),            options.quant_table,
                params.strip.as_ptr(),                  options.strip as i32,
                params.background.as_ptr(),             background_array,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn png_buffer( &self ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();

        unsafe {
            match bindings::vips_pngsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                utils::NULL,
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn png_buffer_opts( &self, options: &PngSaveOptions ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();
        let params = globals::get_params().unwrap();
        let profile = c_string(&options.profile).unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_pngsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                params.compression.as_ptr(),        options.compression,
                params.interlace.as_ptr(),          options.interlace as i32,
                params.page_height.as_ptr(),        options.page_height,
                params.profile.as_ptr(),            profile.as_ptr(),
                params.filter.as_ptr(),             options.filter,
                params.palette.as_ptr(),            options.palette as i32,
                params.colours.as_ptr(),            options.colours,
                params.q.as_ptr(),                  options.q,
                params.dither.as_ptr(),             options.dither,
                params.strip.as_ptr(),              options.strip as i32,
                params.background.as_ptr(),         background_array,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn webp_buffer( &self ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();

        unsafe {
            match bindings::vips_webpsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                utils::NULL,
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn webp_buffer_opts( &self, options: &WebPSaveOptions ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();
        let params = globals::get_params().unwrap();
        let profile = c_string(&options.profile).unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_webpsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                params.page_height.as_ptr(),        options.page_height,
                params.q.as_ptr(),                  options.q,
                params.strip.as_ptr(),              options.strip as i32,
                params.background.as_ptr(),         background_array,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn avif_buffer( &self ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();

        unsafe {
            match bindings::vips_heifsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn avif_buffer_opts( &self, options: &HeifsaveOptions ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: usize = 0;
        let mut buffer_out = null();
        let params = globals::get_params().unwrap();
        let profile = c_string(&options.profile).unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_heifsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                params.page_height.as_ptr(),        options.page_height,
                params.q.as_ptr(),                  options.q,
                params.background.as_ptr(),         background_array,
                params.compression.as_ptr(),        options.compression,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn svg_buffer_opts( &self ) -> Result<Vec<u8>, String> {
        let format = self.get_format().unwrap();

        if format == VipsFormat::SVG {
            // already svg, return raw buffer
            let buffer = self.raw_buffer().unwrap();
            return Ok( buffer );
        }

        let buffer = self.to_buffer().unwrap();

        let [ width, height ] = [ self.get_width(), self.get_height() ];

        let mime_type = match format {
            VipsFormat::JPEG => "image/jpeg",
            VipsFormat::PNG => "image/png",
            VipsFormat::WEBP => "image/webp",
            VipsFormat::AVIF => "image/avif",
            VipsFormat::SVG => "image/svg+xml",
        };

        let svg = format!(
            "<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\"><image href=\"data:{};base64,{}\" width=\"{}\" height=\"{}\"/></svg>",
            width,
            height,
            width,
            height,
            mime_type,
            general_purpose::STANDARD.encode( &buffer ),
            width,
            height
        );

        let svg_bytes = svg.as_bytes().to_vec();
        Ok( svg_bytes )
    }

    pub fn svg_buffer( &self ) -> Result<Vec<u8>, String> {
        return self.svg_buffer_opts();
    }

    pub fn raw_buffer( &self ) -> Result<Vec<u8>, String> {
        match self.source {
            ImageSource::Buffer( ref buffer ) => Ok( buffer.clone() ),
            ImageSource::File( ref path ) => {
                let buffer = fs::read( path ).unwrap();
                Ok( buffer )
            },
            ImageSource::None => Err( "no source".to_string() )
        }
    }

    pub fn resize( &self, scale: f64 ) -> Result<VipsImage, String> {
        unsafe {
            let mut output: *mut bindings::VipsImage = null();

            match bindings::vips_resize(
                self.image as *mut bindings::_VipsImage,
                &mut output,
                scale,
                utils::NULL
            ) {
                0 => Ok( VipsImage{
                    image: output,
                    source: self.source.clone(),
                } ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn to_buffer( &self ) -> Result<Vec<u8>, String> {
        match self.get_format().unwrap() {
            VipsFormat::JPEG => self.jpeg_buffer(),
            VipsFormat::PNG => self.png_buffer(),
            VipsFormat::WEBP => self.webp_buffer(),
            VipsFormat::AVIF => self.avif_buffer(),
            VipsFormat::SVG => self.raw_buffer(),
        }
    }


}

impl Drop for VipsImage {
    fn drop(&mut self) {
        self.destroy()
    }
}