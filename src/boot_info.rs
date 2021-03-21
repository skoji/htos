#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub rgb: [u8; 3usize],
    pub _rsvd: u8,
}

type PixelFormat = cty::c_uint;
pub const PIXEL_FORMAT_RGBCOLOR: PixelFormat = 0;
pub const PIXEL_FORMAT_BGRCOLOR: PixelFormat = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VideoInfo {
    pub format: PixelFormat,
    pub frame_buffer_base: *mut Pixel,
    pub frame_buffer_size: u64,
    pub horizonal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixels_per_scan_line: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BootInfo {
    pub vinfo: VideoInfo,
}

