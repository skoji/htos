use crate::graphics::PixelWriter;
use crate::fonts::write_string;

pub struct Console<'a> {
    pub current_position_x: u32,
    pub current_position_y: u32,
    pub screen_buffer: [[u8; 80]; 40],
    pub buffer_line_tail_index: usize,
    pub pixel_writer: &'a PixelWriter,
    pub screen_line: usize,
    pub total_line: usize,
}

impl<'a> Console<'a> {
    fn write_screen_buffer(&mut self, s: &str) {
        for i in 0..s.len() {
            if i % 80 == 0 {
                self.buffer_line_tail_index = (self.buffer_line_tail_index + 1) % 80;
            }
            self.screen_buffer[self.buffer_line_tail_index][i % 80] = s.as_bytes()[i];
        }
        self.buffer_line_tail_index = (self.buffer_line_tail_index + 1) % 80;
    }

    fn newline(&mut self) {
        self.total_line += 1;
        self.current_position_x = 0;
        if self.screen_line < 40 {
            self.current_position_y += 16;
            self.screen_line += 1;
        }
    }

    fn write_line(&self, s: &str, line_number: u32) {
        write_string(0, line_number * 16, s, self.pixel_writer);
    }

    // 一行画面に出力する関数
    pub fn put_string(&mut self, s: &str) {
        self.write_screen_buffer(s); // バッファに格納

        // 描画部分
        for i in 0..self.screen_line {
            if self.screen_line < 40 {
                self.write_line(core::str::from_utf8(&self.screen_buffer[i]).unwrap(), i as u32);
            } else {
                self.write_line(core::str::from_utf8(&self.screen_buffer[(i + self.buffer_line_tail_index) % 40]).unwrap(), i as u32);
            }
            self.newline();
        }
    }
}
