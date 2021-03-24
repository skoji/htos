use crate::graphics::PixelWriter;
use crate::fonts::write_byte;

// 画面サイズ 80x25
pub const ROW_NUMBER: usize = 25;
pub const COLUNM_NUMBER: usize = 80;

pub struct Console {
    pub position_x: usize, // 横のカーソル位置
    pub position_y: usize, // 縦のカーソル位置
    pub screen_buffer: [[u8; COLUNM_NUMBER]; ROW_NUMBER], // スクリーンバッファ
    pub row_head_index: usize,    // バッファ内のデータの最初の行
    pub row_tail_index: usize,    // バッファ内のデータの最後の行
    pub column_tail_index: usize, // バッファ内のデータの最後の行の最後の列
    pub pixel_writer: PixelWriter, // 画面描画構造体
}

impl Console {
    // バッファの末尾のインデックスを次の行にするメソッド
    fn buffer_newline(&mut self) {
        self.column_tail_index = 0;
        self.row_tail_index = (self.row_tail_index + 1) % ROW_NUMBER;
        if self.row_tail_index == self.row_head_index {
            // もし既存の行を上書きする場合
            // headをインクリメントする
            self.row_head_index = (self.row_head_index + 1) % ROW_NUMBER;
            // 既存の行を'\0'埋めしておく
            for i in 0..COLUNM_NUMBER {
                self.screen_buffer[self.row_tail_index][i] = b'\0';
            }
        }
    }

    // バッファに1文字格納するメソッド
    fn push_screen_buffer(&mut self, c: u8) {
        if c == b'\n' {
            self.buffer_newline();
        } else {
            self.column_tail_index = (self.column_tail_index + 1) % COLUNM_NUMBER;
            if self.column_tail_index == 0 {
                self.buffer_newline();
            }
            self.screen_buffer[self.row_tail_index][self.column_tail_index] = c;
        }
    }

    fn screen_newline(&mut self) {
        self.position_x = 0;
        if self.position_y + 1 < ROW_NUMBER {
            self.position_y += 1;
        } else {
            // 一番下に来ている場合
            // バッファから画面を書き直す
            for y in 0..ROW_NUMBER {
                for x in 0..COLUNM_NUMBER {
                    write_byte(x * 8, y * 16, self.screen_buffer[(self.row_head_index + y) % ROW_NUMBER][x], &self.pixel_writer);
                }
            }
        }
    }

    fn put_char(&mut self, c: u8) {
        if c == b'\n' {
            self.screen_newline();
        } else {
            write_byte(self.position_x * 8, self.position_y * 16, c, &self.pixel_writer);
            self.position_x = (self.position_x + 1) % COLUNM_NUMBER;
            if self.position_x == 0 {
                self.screen_newline();
            }
        }
    }

    // 1文字画面に出力するメソッド
    pub fn put(&mut self, c: u8) {
        self.push_screen_buffer(c);
        self.put_char(c);
    }

    // 一行画面に出力する関数
    pub fn puts(&mut self, s: &str) {
        for i in 0..s.len() {
            self.put(s.as_bytes()[i]);
        }
    }
}
