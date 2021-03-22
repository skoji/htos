use crate::graphics::{PixelWriter, PixelColor};

extern "C" {
    pub static mut fonts: [[u8; 16usize]; 256usize];
}

unsafe fn get_font(character: char) -> &'static [u8; 16usize] {
    let char_idx = character as usize;
    &fonts[char_idx]
}

pub unsafe fn write_font(x: u32, y: u32, character: char, writer: &PixelWriter) {
    let font_slice = get_font(character);
    for dy in 0..16 {
        for dx in 0..8 {
            if ((font_slice[dy] << dx) & 0x80) == 1 {
                writer.write(x + dx as u32, y + dy as u32, &PixelColor::White);
            } else {
                writer.write(x + dx as u32, y + dy as u32, &PixelColor::Black);
            }
        }
    }
}
