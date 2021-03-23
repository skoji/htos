#![no_main]
#![no_std]

use core::panic::PanicInfo;

extern crate htos;
use htos::boot_info::BootInfo;
use htos::graphics::{PixelWriter, PixelColor};
use htos::fonts::write_string;

extern "C" {
    pub static mut fonts: [[u8; 16usize]; 256usize];
}

#[link_section = ".text.entry"]
#[no_mangle]
pub unsafe extern "C" fn kernel_entry(binfo: *mut BootInfo) -> ! {
    let pixel_writer = PixelWriter { video_info: &mut (*binfo).vinfo };
    pixel_writer.write_background(PixelColor::Black);
    write_string(0, 0, "hello, HTOS!", &pixel_writer);
    loop {}
}

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
