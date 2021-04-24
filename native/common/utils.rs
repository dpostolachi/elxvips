use std::ffi::CString;
use std::ptr::null_mut;
use std::ffi::{c_void};

pub const NULL: *const c_void = null_mut();

pub fn null<T>() -> *mut T {
    null_mut()
}

pub fn c_string(string: &str) -> Result<CString, &str> {
    match CString::new( string ) {
        Ok( c_ref ) => Ok( c_ref ),
        Err( _ ) => Err( "failed to initialiaze c_string" )
    }
}

pub fn get_buffer(buf: *mut c_void, size: u64) -> Vec<u8> {
    unsafe {
        Vec::from_raw_parts(buf as *mut u8, size as usize, size as usize)
    }
}