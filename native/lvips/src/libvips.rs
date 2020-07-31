#![allow(unused)]
pub mod bindings;
pub mod utils;
pub mod save_options;
pub mod globals;
use std::ffi::{CStr, c_void};
use utils::{c_string, null};
use std::ffi::{CString};
use save_options::{JpegSaveOptions, PngSaveOptions, SmartcropOptions};

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

pub struct VipsImage {
    image: *mut bindings::_VipsImage,
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
    pub fn from_file( path: &str ) -> Result<VipsImage, String> {
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
    pub fn from_buffer( buffer: &[u8] ) -> Result<VipsImage, String> {
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
    pub fn crop( &self, left: i32, top: i32, width: i32, height: i32 ) -> Result<VipsImage, String> {
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
    pub fn smart_crop( &self, width: i32, height: i32 ) -> Result<VipsImage, String> {
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

    pub fn jpeg_buffer_opts( &self, options: &JpegSaveOptions ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: u64 = 0;
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

    pub fn png_buffer_opts( &self, options: &PngSaveOptions ) -> Result<Vec<u8>, String> {
        let mut buffer_buf_size: u64 = 0;
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
                } ),
                _ => Err( error_buffer() )
            }
        }
    }

}

impl Drop for VipsImage {
    fn drop(&mut self) {
        self.destroy()
    }
}