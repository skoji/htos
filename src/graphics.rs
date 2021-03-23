//use core::mem::replace;

use crate::boot_info::{VideoInfo, Pixel};

//struct Graphics {
//    pixel_writer: Option<PixelWriter>,
//}
//
//impl Graphics {
//    fn take_pixel_writer(&mut self) -> PixelWriter {
//        let p = replace(&mut self.pixel_writer, None);
//        p.unwrap()
//    }
//}
//
//static mut GRAPHICS: Graphics = Graphics {
//    pixel_writer: Some(PixelWriter)
//};

pub enum PixelColor {
    Black,
    White,
}

pub struct PixelWriter {
    pub video_info: &'static mut VideoInfo,
}

impl PixelWriter {
    pub fn write(&self, x: u32, y: u32, color: &PixelColor) {
        if x >= self.video_info.horizonal_resolution && y >= self.video_info.vertical_resolution {
            loop {}
        }
        unsafe {
            match color {
                PixelColor::Black => *self.video_info.frame_buffer_base.offset((y * self.video_info.horizonal_resolution + x) as isize) = Pixel { rgb: [0x00, 0x00, 0x00], _rsvd: 0x00 },
                PixelColor::White => *self.video_info.frame_buffer_base.offset((y * self.video_info.horizonal_resolution + x) as isize) = Pixel { rgb: [0xff, 0xff, 0xff], _rsvd: 0x00 },
            }
        }
    }

    pub fn write_background(&self, color: PixelColor) {
        for x in 0..self.video_info.horizonal_resolution {
            for y in 0..self.video_info.vertical_resolution {
                self.write(x, y, &color);
            }
        }
    }
}
