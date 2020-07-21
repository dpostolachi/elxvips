#![allow(unused)]
pub mod bindings;
pub mod utils;
pub mod save_options;
use std::ffi::{CStr, c_void};
use utils::{c_string, null};
use lazy_static::lazy_static;
use std::ffi::{CString};
use save_options::{JpegSaveOptions, PngSaveOptions, SmartcropOptions};

pub fn error_buffer() -> &'static str {
    unsafe {
        let buffer = CStr::from_ptr(bindings::vips_error_buffer());
        buffer
            .to_str()
            .unwrap()
    }
}

pub fn concurrency_set( max: i32 ) {
    unsafe {
        bindings::vips_concurrency_set(max);
    }
}

pub struct VipsImage {
    image: *mut bindings::_VipsImage,
}

lazy_static! {
    static ref PAR_PAGE_HEIGHT:         CString = c_string( "page_height" ).unwrap();
    static ref PAR_Q:                   CString = c_string("Q").unwrap();
    static ref PAR_PROFILE:             CString = c_string("profile").unwrap();
    static ref PAR_OPTIMIZE_CODING:     CString = c_string("optimize-coding").unwrap();
    static ref PAR_INTERLACE:           CString = c_string("interlace").unwrap();
    static ref PAR_NO_SUB_SAMPLE:       CString = c_string("no-subsample").unwrap();
    static ref PAR_TRELLIS_QUANT:       CString = c_string("trellis-quant").unwrap();
    static ref PAR_OVERSHOOT_DERINGING: CString = c_string("overshoot-deringing").unwrap();
    static ref PAR_OPTIMIZE_SCANS:      CString = c_string("optimize-scans").unwrap();
    static ref PAR_QUANT_TABLE:         CString = c_string("quant-table").unwrap();
    static ref PAR_STRIP:               CString = c_string("strip").unwrap();
    static ref PAR_BACKGROUND:          CString = c_string("background").unwrap();

    static ref PAR_COMPRESSION:         CString = c_string("compression").unwrap();
    static ref PAR_FILTER:              CString = c_string("filter").unwrap();
    static ref PAR_PALETTE:             CString = c_string("palette").unwrap();
    static ref PAR_COLOURS:             CString = c_string("colours").unwrap();
    static ref PAR_DITHER:              CString = c_string("dither").unwrap();

    static ref PAR_INTERESTING:          CString = c_string("interesting").unwrap();
}

impl VipsImage {
    pub fn get_width( &self ) -> i32 {
        unsafe {
            bindings::vips_image_get_width( self.image )
        }
    }
    pub fn get_height( &self ) -> i32 {
        unsafe {
            bindings::vips_image_get_height( self.image )
        }
    }
    pub fn from_file( path: &str ) -> Result<VipsImage, &str> {
        let filename = c_string( path ).unwrap();
        unsafe {

            let image = bindings::vips_image_new_from_file( filename.as_ptr(), utils::NULL );

            if image.is_null() {
                Err( error_buffer() )
            } else {
                Ok( VipsImage{
                    image: image,
                } )
            }
        }
    }
    pub fn from_buffer( buffer: &[u8] ) -> Result<VipsImage, &str> {

        let options = c_string("").unwrap();
        unsafe {
            let image = bindings::vips_image_new_from_buffer(
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
                options.as_ptr(),
                utils::NULL
            );

            if image.is_null() {
                Err( error_buffer() )
            } else {
                Ok( VipsImage{
                    image: image,
                } )
            }
        }

    }
    pub fn crop( &self, left: i32, top: i32, width: i32, height: i32 ) -> Result<VipsImage, &str> {
        let input: *mut bindings::VipsImage = self.image;
        let mut output: *mut bindings::VipsImage = null();
        unsafe {
            match bindings::vips_crop( input, &mut output, left, top, width, height, utils::NULL ) {
                0 => Ok( VipsImage{
                    image: output,
                } ),
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn smart_crop( &self, width: i32, height: i32 ) -> Result<VipsImage, &str> {
        let input: *mut bindings::VipsImage = self.image;
        let mut output: *mut bindings::VipsImage = null();
        unsafe {
            match bindings::vips_smartcrop( input, &mut output, width, height, utils::NULL ) {
                0 => Ok( VipsImage{
                    image: output,
                } ),
                _ => Err( error_buffer() )
            }
        } 
    }

    pub fn smart_crop_opts( &self, width: i32, height: i32, options: &SmartcropOptions ) -> Result<VipsImage, &str> {
        let input: *mut bindings::VipsImage = self.image;
        let mut output: *mut bindings::VipsImage = null();
        unsafe {
            match bindings::vips_smartcrop(
                input,
                &mut output,
                width,
                height,
                PAR_INTERESTING.as_ptr(),       options.interesting as i32,
                utils::NULL
            ) {
                0 => Ok( VipsImage{
                    image: output,
                } ),
                _ => Err( error_buffer() )
            }
        }  
    }

    pub fn save_png( &self, path: &str ) -> Result<(), &str> {
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
    pub fn save_png_opts( &self, path: &str, options: &save_options::PngSaveOptions ) -> Result<(), &str> {
        let filename = c_string( path ).unwrap();
        let profile = c_string(&options.profile).unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_pngsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                PAR_COMPRESSION.as_ptr(),        options.compression,
                PAR_INTERLACE.as_ptr(),          options.interlace as i32,
                PAR_PAGE_HEIGHT.as_ptr(),        options.page_height,
                PAR_PROFILE.as_ptr(),            profile.as_ptr(),
                PAR_FILTER.as_ptr(),             options.filter,
                PAR_PALETTE.as_ptr(),            options.palette as i32,
                PAR_COLOURS.as_ptr(),            options.colours,
                PAR_Q.as_ptr(),                  options.q,
                PAR_DITHER.as_ptr(),             options.dither,
                PAR_STRIP.as_ptr(),              options.strip as i32,
                PAR_BACKGROUND.as_ptr(),         background_array,
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }
    pub fn save_jpeg( &self, path: &str ) -> Result<(), &str> {
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
    pub fn save_jpeg_opts( &self, path: &str, options: &JpegSaveOptions ) -> Result<(), &str> {
        let filename = c_string( path ).unwrap();
        let profile = c_string(&options.profile).unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_jpegsave(
                self.image as *mut bindings::_VipsImage,
                filename.as_ptr(),
                PAR_PAGE_HEIGHT.as_ptr(),            options.page_height,
                PAR_Q.as_ptr(),                      options.q,
                PAR_PROFILE.as_ptr(),                profile.as_ptr(),
                PAR_OPTIMIZE_CODING.as_ptr(),        options.optimize_coding as i32,
                PAR_INTERLACE.as_ptr(),              options.interlace as i32,
                PAR_NO_SUB_SAMPLE.as_ptr(),          options.no_subsample as i32,
                PAR_TRELLIS_QUANT.as_ptr(),          options.trellis_quant as i32,
                PAR_OVERSHOOT_DERINGING.as_ptr(),    options.overshoot_deringing as i32,
                PAR_OPTIMIZE_SCANS.as_ptr(),         options.optimize_scans as i32,
                PAR_QUANT_TABLE.as_ptr(),            options.quant_table,
                PAR_STRIP.as_ptr(),                  options.strip as i32,
                PAR_BACKGROUND.as_ptr(),             background_array,
                utils::NULL
            ) {
                0 => Ok( () ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn jpeg_buffer( &self ) -> Result<Vec<u8>, &str> {
        let mut buffer_buf_size: u64 = 0;
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

    pub fn jpeg_buffer_opts( &self, options: &JpegSaveOptions ) -> Result<Vec<u8>, &str> {
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out = null();
        let profile = c_string(&options.profile).unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_jpegsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                PAR_PAGE_HEIGHT.as_ptr(),            options.page_height,
                PAR_Q.as_ptr(),                      options.q,
                PAR_PROFILE.as_ptr(),                profile.as_ptr(),
                PAR_OPTIMIZE_CODING.as_ptr(),        options.optimize_coding as i32,
                PAR_INTERLACE.as_ptr(),              options.interlace as i32,
                PAR_NO_SUB_SAMPLE.as_ptr(),          options.no_subsample as i32,
                PAR_TRELLIS_QUANT.as_ptr(),          options.trellis_quant as i32,
                PAR_OVERSHOOT_DERINGING.as_ptr(),    options.overshoot_deringing as i32,
                PAR_OPTIMIZE_SCANS.as_ptr(),         options.optimize_scans as i32,
                PAR_QUANT_TABLE.as_ptr(),            options.quant_table,
                PAR_STRIP.as_ptr(),                  options.strip as i32,
                PAR_BACKGROUND.as_ptr(),             background_array,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn png_buffer( &self ) -> Result<Vec<u8>, &str> {
        let mut buffer_buf_size: u64 = 0;
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

    pub fn png_buffer_opts( &self, options: &PngSaveOptions ) -> Result<Vec<u8>, &str> {
        let mut buffer_buf_size: u64 = 0;
        let mut buffer_out = null();
        let profile = c_string(&options.profile).unwrap();

        unsafe {
            let background_array = bindings::vips_array_double_new(options.background.as_ptr(), options.background.len() as i32);

            match bindings::vips_pngsave_buffer(
                self.image as *mut bindings::_VipsImage,
                &mut buffer_out,
                &mut buffer_buf_size,
                PAR_COMPRESSION.as_ptr(),    options.compression,
                PAR_INTERLACE.as_ptr(),      options.interlace as i32,
                PAR_PAGE_HEIGHT.as_ptr(),    options.page_height,
                PAR_PROFILE.as_ptr(),        profile.as_ptr(),
                PAR_FILTER.as_ptr(),         options.filter,
                PAR_PALETTE.as_ptr(),        options.palette as i32,
                PAR_COLOURS.as_ptr(),        options.colours,
                PAR_Q.as_ptr(),              options.q,
                PAR_DITHER.as_ptr(),         options.dither,
                PAR_STRIP.as_ptr(),          options.strip as i32,
                PAR_BACKGROUND.as_ptr(),     background_array,
                utils::NULL
            ) {
                0 => Ok( utils::get_buffer( buffer_out, buffer_buf_size ) ),
                _ => Err( error_buffer() )
            }
        }
    }

    pub fn resize( &self, scale: f64 ) -> Result<VipsImage, &str> {
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
                } ),
                _ => Err( error_buffer() )
            }
        }
    }

}