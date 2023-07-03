use std::sync::{Once};
use std::ffi::{CString};
use super::utils::{c_string};

pub struct GlobalsParams {
    pub page_height:            CString,
    pub q:                      CString,
    pub profile:                CString,
    pub optimize_coding:        CString,
    pub interlace:              CString,
    pub no_sub_sample:          CString,
    pub trellis_quant:          CString,
    pub overshoot_deringing:    CString,
    pub optimize_scans:         CString,
    pub quant_table:            CString,
    pub strip:                  CString,
    pub background:             CString,

    pub compression:            CString,
    pub filter:                 CString,
    pub palette:                CString,
    pub colours:                CString,
    pub dither:                 CString,

    pub interesting:            CString,

    pub vips_loader:            CString,

    pub n:                      CString,
    pub page:                      CString,
 
}

impl Default for GlobalsParams {
    fn default() -> Self {
        GlobalsParams {
            page_height:            c_string( "page_height" ).unwrap(),
            q:                      c_string( "Q" ).unwrap(),
            profile:                    c_string( "profile" ).unwrap(),
            optimize_coding:        c_string( "optimize-coding" ).unwrap(),
            interlace:              c_string( "interlace" ).unwrap(),
            no_sub_sample:          c_string( "no-subsample" ).unwrap(),
            trellis_quant:          c_string( "trellis-quant" ).unwrap(),
            overshoot_deringing:    c_string( "overshoot-deringing" ).unwrap(),
            optimize_scans:         c_string( "optimize-scans" ).unwrap(),
            quant_table:            c_string( "quant-table" ).unwrap(),
            strip:                  c_string( "strip" ).unwrap(),

            background:             c_string( "background" ).unwrap(),
            compression:            c_string( "compression" ).unwrap(),
            filter:                 c_string( "filter" ).unwrap(),
            palette:                c_string( "palette" ).unwrap(),
            colours:                c_string( "colours" ).unwrap(),
            dither:                 c_string( "dither" ).unwrap(),

            interesting:            c_string( "interesting" ).unwrap(),

            vips_loader:            c_string( "vips-loader" ).unwrap(),

            n:                      c_string( "n" ).unwrap(),
            page:                   c_string( "page" ).unwrap(),
        }
    }
}

static mut GLOBAL_PARAMS: Option<GlobalsParams> = None;

pub fn get_params() -> Result<&'static GlobalsParams, &'static str> {
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            GLOBAL_PARAMS = Some(GlobalsParams::default())
        });

        match &GLOBAL_PARAMS {
            Some( params ) => Ok( &params ),
            None => Err( "failed to get params" )
        }
    }
}
