#![no_main]
#![no_std]

use core::panic::PanicInfo;

extern crate htos;
use htos::boot_info::BootInfo;
use htos::graphics::{PixelWriter, PixelColor};
use htos::console::Console;

extern "C" {
    pub static mut fonts: [[u8; 16usize]; 256usize];
}

#[link_section = ".text.entry"]
#[no_mangle]
pub unsafe extern "C" fn kernel_entry(binfo: *mut BootInfo) -> ! {
    let pixel_writer = PixelWriter { video_info: &mut (*binfo).vinfo };
    pixel_writer.write_background(PixelColor::Black);
    let mut console = Console {
        current_position_x: 0,
        current_position_y: 0,
        screen_buffer: [[0; 80]; 40],
        buffer_line_tail_index: 0,
        pixel_writer: &pixel_writer,
        screen_line: 1,
        total_line: 1,
    };
    console.put_string("Hello");
    console.put_string("Hyper");
    console.put_string("Toy");
    console.put_string("OS");
    console.put_string("!!!!!");
    loop {}
}

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
