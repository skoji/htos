#![no_main]
#![no_std]

use core::panic::PanicInfo;

use htos::boot_info::BootInfo;
use htos::graphics::{PixelWriter, PixelColor};
use htos::console::{Console, ROW_NUMBER, COLUNM_NUMBER};

#[link_section = ".text.entry"]
#[no_mangle]
pub extern "C" fn kernel_entry(binfo: *mut BootInfo) -> ! {
    let pixel_writer = unsafe { PixelWriter { video_info: &mut (*binfo).vinfo } };
    pixel_writer.write_background(PixelColor::Black);
    let mut console = Console {
        position_x: 0,
        position_y: 0,
        screen_buffer: [[b'\0'; COLUNM_NUMBER]; ROW_NUMBER],
        row_head_index: 0,
        row_tail_index: 0,
        column_tail_index: 0,
        pixel_writer,
    };

    for _ in 0..(ROW_NUMBER - 1 + 2) {
        console.put(b'0');
        console.put(b'\n');
    }

    loop {}
}

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
