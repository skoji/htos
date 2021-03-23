use crate::graphics::PixelWriter;
use crate::fonts::write_string;

pub struct Console<'a> {
    pub current_position_x: u32,
    pub current_position_y: u32,
    pub screen_buffer: [[u8; 80]; 40],
    pub buffer_line_tail_index: usize,
    pub pixel_writer: &'a PixelWriter,
}

impl<'a> Console<'a> {
    fn write_screen_buffer(&mut self, s: &str) {
        self.buffer_line_tail_index = (self.buffer_line_tail_index + 1) % 40;
        for i in 0..s.len() {
            if i % 40 == 0 {
                self.buffer_line_tail_index = (self.buffer_line_tail_index + 1) % 40;
            }
            self.screen_buffer[self.buffer_line_tail_index][i % 40] = s.as_bytes()[i];
        }
    }

    fn newline(&mut self) {
        self.current_position_x = 0;
        if self.current_position_y / 16 != 40 {
            self.current_position_y += 16;
        }
    }

    fn write_line(&self, s: &str) {
        unsafe {
            write_string(0, self.current_position_y, s, self.pixel_writer);
        }
    }

    pub fn put_string(&mut self, s: &str) {
        self.write_screen_buffer(s);
        for i in 0..40 {
            self.write_line(core::str::from_utf8(&self.screen_buffer[i]).unwrap());
            self.newline();
        }
    }
}
