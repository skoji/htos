use crate::graphics::{PixelWriter, PixelColor};
use crate::fonts::write_byte;

// 画面サイズ 80x25
pub const ROW_NUMBER: usize = 25;
pub const COLUNM_NUMBER: usize = 80;

pub struct Console<'a> {
    pub position_x: usize, // 横のカーソル位置
    pub position_y: usize, // 縦のカーソル位置
    pub screen_buffer: [u8; COLUNM_NUMBER * ROW_NUMBER], // スクリーンバッファ
    pub data_head: usize, // バッファ上の一番最初のデータがあるインデックス
    pub data_len: usize, // バッファにあるデータの長さ
    pub pixel_writer: &'a PixelWriter, // 画面描画構造体
}

impl<'a> Console<'a> {
    // バッファに1文字格納するメソッド
    fn set_screen_buffer(&mut self, c: u8) {
        if self.data_len < COLUNM_NUMBER * ROW_NUMBER {
            // データがバッファからあふれていない場合
            self.screen_buffer[self.data_head + self.data_len] = c;
            self.data_len += 1;
        } else {
            // データがいっぱいになった場合
            self.screen_buffer[(self.data_head + self.data_len) % (COLUNM_NUMBER + ROW_NUMBER)] = c;
            self.data_head = (self.data_head + 1) % (COLUNM_NUMBER + ROW_NUMBER);
        }
    }

    fn newline(&mut self) {
        self.position_x = 0;
        if self.position_y + 1 < ROW_NUMBER {
            self.position_y += 1;
        } else {
            // 画面を黒で塗りつぶす
            for y in 0..(ROW_NUMBER * 16) {
                for x in 0..(COLUNM_NUMBER * 8) {
                    self.pixel_writer.write(x, y, &PixelColor::Black);
                }
            }
            // TODO: 文字を書き直す
        }
    }

    fn put_char(&mut self, c: u8) {
        if c == b'\n' {
            self.newline();
        } else {
            write_byte(self.position_x * 8, self.position_y * 16, c, self.pixel_writer);
            if self.position_x + 1 < COLUNM_NUMBER {
                self.position_x += 1;
            } else {
                self.newline();
            }
        }
    }

    // 1文字画面に出力するメソッド
    pub fn put(&mut self, c: u8) {
        self.set_screen_buffer(c);
        self.put_char(self.screen_buffer[self.data_head + self.data_len - 1]);
    }

    // 一行画面に出力する関数
    pub fn puts(&mut self, s: &str) {
        for i in 0..s.len() {
            self.put(s.as_bytes()[i]);
        }
    }
}
