// copied from https://github.com/augustocdias/libvips-rust-bindings/blob/master/src/ops.rs

/// Options for jpegsave operation
#[derive(Clone, Debug)]
pub struct JpegSaveOptions {
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 75
    pub q: i32,
    /// profile: `String` -> ICC profile to embed
    pub profile: String,
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    /// default: false
    pub optimize_coding: bool,
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    /// default: false
    pub interlace: bool,
    /// no_subsample: `bool` -> Disable chroma subsample
    /// default: false
    pub no_subsample: bool,
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    /// default: false
    pub trellis_quant: bool,
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    /// default: false
    pub overshoot_deringing: bool,
    /// optimize_scans: `bool` -> Split the spectrum of DCT coefficients into separate scans
    /// default: false
    pub optimize_scans: bool,
    /// quant_table: `i32` -> Use predefined quantization table with given index
    /// min: 0, max: 8, default: 0
    pub quant_table: i32,
    /// strip: `bool` -> Strip all metadata from image
    /// default: false
    pub strip: bool,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
}

impl std::default::Default for JpegSaveOptions {
    fn default() -> Self {
        JpegSaveOptions {
            page_height: i32::from(0),
            q: i32::from(75),
            profile: String::from("sRGB"),
            optimize_coding: false,
            interlace: false,
            no_subsample: false,
            trellis_quant: false,
            overshoot_deringing: false,
            optimize_scans: false,
            quant_table: i32::from(0),
            strip: false,
            background: Vec::new(),
        }
    }
}

/// Options for pngsave operation
#[derive(Clone, Debug)]
pub struct PngSaveOptions {
    /// compression: `i32` -> Compression factor
    /// min: 0, max: 9, default: 6
    pub compression: i32,
    /// interlace: `bool` -> Interlace image
    /// default: false
    pub interlace: bool,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> ICC profile to embed
    pub profile: String,
    /// filter: `ForeignPngFilter` -> libpng row filter flag(s)
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248 [DEFAULT]
    pub filter: ForeignPngFilter,
    /// palette: `bool` -> Quantise to 8bpp palette
    /// default: false
    pub palette: bool,
    /// colours: `i32` -> Max number of palette colours
    /// min: 2, max: 256, default: 256
    pub colours: i32,
    /// q: `i32` -> Quantisation quality
    /// min: 0, max: 100, default: 100
    pub q: i32,
    /// dither: `f64` -> Amount of dithering
    /// min: 0, max: 1, default: 1
    pub dither: f64,
    /// strip: `bool` -> Strip all metadata from image
    /// default: false
    pub strip: bool,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
}

impl std::default::Default for PngSaveOptions {
    fn default() -> Self {
        PngSaveOptions {
            compression: i32::from(6),
            interlace: false,
            page_height: i32::from(0),
            profile: String::from("sRGB"),
            filter: ForeignPngFilter::All,
            palette: false,
            colours: i32::from(256),
            q: i32::from(100),
            dither: f64::from(1),
            strip: false,
            background: Vec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ForeignPngFilter {
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8
    None = 8,
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    Sub = 16,
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    Up = 32,
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    Avg = 64,
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    Paeth = 128,
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248
    All = 248,
}
