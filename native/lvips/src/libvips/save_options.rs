#![allow(unused)]
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

/// Options for webpsave operation
#[derive(Clone, Debug)]
pub struct WebPSaveOptions {
    /// q: `i32` -> Q factor
    /// min: 0, max: 100, default: 75
    pub q: i32,
    /// lossless: `bool` -> enable lossless compression
    /// default: false
    pub lossless: bool,
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0 [DEFAULT]
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    pub preset: ForeignWebpPreset,
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    /// default: false
    pub smart_subsample: bool,
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    /// default: false
    pub near_lossless: bool,
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    /// min: 0, max: 100, default: 100
    pub alpha_q: i32,
    /// min_size: `bool` -> Optimise for minium size
    /// default: false
    pub min_size: bool,
    /// kmin: `i32` -> Minimum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483646
    pub kmin: i32,
    /// kmax: `i32` -> Maximum number of frames between key frames
    /// min: 0, max: 2147483647, default: 2147483647
    pub kmax: i32,
    /// reduction_effort: `i32` -> Level of CPU effort to reduce file size
    /// min: 0, max: 6, default: 4
    pub reduction_effort: i32,
    /// profile: `String` -> ICC profile to embed
    pub profile: String,
    /// strip: `bool` -> Strip all metadata from image
    /// default: false
    pub strip: bool,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
}

impl std::default::Default for WebPSaveOptions {
    fn default() -> Self {
        WebPSaveOptions {
            q: i32::from(75),
            lossless: false,
            preset: ForeignWebpPreset::Default,
            smart_subsample: false,
            near_lossless: false,
            alpha_q: i32::from(100),
            min_size: false,
            kmin: i32::from(2147483646),
            kmax: i32::from(2147483647),
            reduction_effort: i32::from(4),
            profile: String::from("sRGB"),
            strip: false,
            background: Vec::new(),
            page_height: i32::from(0),
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

#[derive(Copy, Clone, Debug)]
pub enum ForeignWebpPreset {
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0
    Default = 0,
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    Picture = 1,
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    Photo = 2,
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    Drawing = 3,
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    Icon = 4,
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    Text = 5,
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug)]
pub enum Interesting {
    ///  `None` -> VIPS_INTERESTING_NONE = 0
    None = 0,
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    Centre = 1,
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    Entropy = 2,
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    Attention = 3,
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    Low = 4,
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    High = 5,
    ///  `Last` -> VIPS_INTERESTING_LAST = 6
    Last = 6,
}

/// Options for smartcrop operation
#[derive(Clone, Debug)]
pub struct SmartcropOptions {
    /// interesting: `Interesting` -> How to measure interestingness
    ///  `None` -> VIPS_INTERESTING_NONE = 0
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3 [DEFAULT]
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    ///  `Last` -> VIPS_INTERESTING_LAST = 6
    pub interesting: Interesting,
}

impl std::default::Default for SmartcropOptions {
    fn default() -> Self {
        SmartcropOptions {
            interesting: Interesting::Attention,
        }
    }
}
