use crate::graphics::{PixelWriter, PixelColor};

extern "C" {
    pub static mut fonts: [[u8; 16usize]; 256usize];
}

unsafe fn get_font(character: u8) -> &'static [u8; 16usize] {
    let char_idx = character as usize;
    &fonts[char_idx]
}

pub fn write_byte(x: usize, y: usize, character: u8, writer: &PixelWriter) {
    let font_slice = unsafe { get_font(character) };
    for dy in 0..16 {
        for dx in 0..8 {
            if ((font_slice[dy] << dx) & 0b10000000) == 0b10000000 {
                writer.write(x + dx, y + dy, &PixelColor::White);
            } else {
                writer.write(x + dx, y + dy, &PixelColor::Black);
            }
        }
    }
}

pub fn write_string(x: usize, y: usize, s: &str, writer: &PixelWriter) {
    let mut dx = 0;
    for i in 0..s.len() {
        write_byte(x + dx, y, s.as_bytes()[i], writer);
        dx += 8;
    }
}
