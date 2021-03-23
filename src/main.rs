#![no_main]
#![no_std]

use core::panic::PanicInfo;

extern crate htos;
use htos::boot_info::BootInfo;
use htos::graphics::{PixelWriter, PixelColor};
use htos::console::{Console, LINE_NUMBER, CHAR_NUMBER_IN_A_LINE};

extern "C" {
    pub static mut fonts: [[u8; 16usize]; 256usize];
}

#[link_section = ".text.entry"]
#[no_mangle]
pub extern "C" fn kernel_entry(binfo: *mut BootInfo) -> ! {
    let pixel_writer = unsafe { PixelWriter { video_info: &mut (*binfo).vinfo } };
    pixel_writer.write_background(PixelColor::Black);
    let mut console = Console {
        current_position_x: 0,
        current_position_y: 0,
        screen_buffer: [[0; CHAR_NUMBER_IN_A_LINE]; LINE_NUMBER],
        buffer_line_tail_index: 0,
        pixel_writer: &pixel_writer,
        screen_line: 1,
        total_line: 1,
    };

    console.put_string("00");
    console.put_string("01");
    console.put_string("02");
    console.put_string("03");
    console.put_string("04");
    console.put_string("05");
    console.put_string("06");
    console.put_string("07");
    console.put_string("08");
    console.put_string("09");
    console.put_string("10");
    console.put_string("11");
    console.put_string("12");
    console.put_string("13");
    console.put_string("14");
    console.put_string("15");
    console.put_string("16");
    console.put_string("17");
    console.put_string("18");
    console.put_string("19");
    console.put_string("20");
    console.put_string("21");
    console.put_string("22");
    console.put_string("23");
    console.put_string("24");
    console.put_string("25");
    console.put_string("26");
    console.put_string("27");
    console.put_string("28");
    console.put_string("29");
    console.put_string("30");
    console.put_string("31");
    console.put_string("32");
    console.put_string("33");
    console.put_string("34");
    console.put_string("35");
    console.put_string("36");
    console.put_string("37");
    console.put_string("38");
    console.put_string("39");
    console.put_string("40");
    console.put_string("41");
    console.put_string("42");
    console.put_string("43");
    console.put_string("44");
    console.put_string("45");
    console.put_string("46");
    console.put_string("47");
    console.put_string("48");
    console.put_string("49");
    console.put_string("50");
    console.put_string("51");
    console.put_string("52");
    console.put_string("53");
    console.put_string("54");
    console.put_string("55");
    console.put_string("56");
    console.put_string("57");
    console.put_string("58");
    console.put_string("59");

    loop {}
}

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
