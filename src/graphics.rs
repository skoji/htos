use crate::boot_info::{VideoInfo, Pixel};

pub enum PixelColor {
    Black,
    White,
}

pub struct PixelWriter {
    pub video_info: &'static mut VideoInfo,
}

impl PixelWriter {
    pub fn write(&self, x: usize, y: usize, color: &PixelColor) {
        if x as u32 >= self.video_info.horizonal_resolution && y as u32 >= self.video_info.vertical_resolution {
            loop {}
        }
        unsafe {
            match color {
                PixelColor::Black => *self.video_info.frame_buffer_base.offset((y * self.video_info.horizonal_resolution as usize + x) as isize) = Pixel { rgb: [0x00, 0x00, 0x00], _rsvd: 0x00 },
                PixelColor::White => *self.video_info.frame_buffer_base.offset((y * self.video_info.horizonal_resolution as usize + x) as isize) = Pixel { rgb: [0xff, 0xff, 0xff], _rsvd: 0x00 },
            }
        }
    }

    pub fn write_background(&self, color: PixelColor) {
        for x in 0..self.video_info.horizonal_resolution as usize {
            for y in 0..self.video_info.vertical_resolution as usize {
                self.write(x, y, &color);
            }
        }
    }
}
