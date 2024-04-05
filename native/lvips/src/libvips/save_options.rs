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

#[derive(Copy, Clone, Debug)]
pub enum ForeignHeifCompression {
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1
    Hevc = 1,
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    Avc = 2,
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    Jpeg = 3,
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    Av1 = 4,
    ///  `Last` -> VIPS_FOREIGN_HEIF_COMPRESSION_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug)]
pub enum ForeignHeifEncoder {
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0
    Auto = 0,
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    Aom = 1,
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    Rav1E = 2,
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    Svt = 3,
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    X265 = 4,
    ///  `Last` -> VIPS_FOREIGN_HEIF_ENCODER_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug)]
pub enum ForeignKeep {
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    None = 0,
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    Exif = 1,
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    Xmp = 2,
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    Iptc = 4,
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    Icc = 8,
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    Other = 16,
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31
    All = 31,
}

#[derive(Copy, Clone, Debug)]
pub enum ForeignSubsample {
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0
    Auto = 0,
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    On = 1,
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    Off = 2,
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    Last = 3,
}

/// Options for heifsave operation
#[derive(Clone, Debug)]
pub struct HeifsaveOptions {
    /// q: `i32` -> Q factor
    /// min: 1, max: 100, default: 50
    pub q: i32,
    /// bitdepth: `i32` -> Number of bits per pixel
    /// min: 1, max: 16, default: 12
    pub bitdepth: i32,
    /// lossless: `bool` -> Enable lossless compression
    /// default: false
    pub lossless: bool,
    /// compression: `ForeignHeifCompression` -> Compression format
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1 [DEFAULT]
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_COMPRESSION_LAST = 5
    pub compression: ForeignHeifCompression,
    /// effort: `i32` -> CPU effort
    /// min: 0, max: 9, default: 4
    pub effort: i32,
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0 [DEFAULT]
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    pub subsample_mode: ForeignSubsample,
    /// encoder: `ForeignHeifEncoder` -> Select encoder to use
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0 [DEFAULT]
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    ///  `Last` -> VIPS_FOREIGN_HEIF_ENCODER_LAST = 5
    pub encoder: ForeignHeifEncoder,
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31 [DEFAULT]
    pub keep: ForeignKeep,
    /// background: `Vec<f64>` -> Background value
    pub background: Vec<f64>,
    /// page_height: `i32` -> Set page height for multipage save
    /// min: 0, max: 10000000, default: 0
    pub page_height: i32,
    /// profile: `String` -> Filename of ICC profile to embed
    pub profile: String,
}

impl std::default::Default for HeifsaveOptions {
    fn default() -> Self {
        HeifsaveOptions {
            q: i32::from(50),
            bitdepth: i32::from(12),
            lossless: false,
            compression: ForeignHeifCompression::Hevc,
            effort: i32::from(4),
            subsample_mode: ForeignSubsample::Auto,
            encoder: ForeignHeifEncoder::Auto,
            keep: ForeignKeep::All,
            background: Vec::new(),
            page_height: i32::from(0),
            profile: String::from("sRGB"),
        }
    }
}


impl std::default::Default for SmartcropOptions {
    fn default() -> Self {
        SmartcropOptions {
            interesting: Interesting::Attention,
        }
    }
}
