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
        screen_buffer: [b'\0'; COLUNM_NUMBER * ROW_NUMBER],
        data_head: 0,
        data_len: 0,
        pixel_writer: &pixel_writer,
    };

    console.puts("00\n");
    console.puts("01\n");
    console.puts("02\n");
    console.puts("03\n");
    console.puts("04\n");
    console.puts("05\n");
    console.puts("06\n");
    console.puts("07\n");
    console.puts("08\n");
    console.puts("09\n");
    console.puts("10\n");
    console.puts("11\n");
    console.puts("12\n");
    console.puts("13\n");
    console.puts("14\n");
    console.puts("15\n");
    console.puts("16\n");
    console.puts("17\n");
    console.puts("18\n");
    console.puts("19\n");
    console.puts("20\n");
    console.puts("21\n");
    console.puts("22\n");
    console.puts("23\n");
    console.puts("24\n");
    console.puts("25\n");
    console.puts("26\n");
    console.puts("27\n");
    console.puts("28\n");
    console.puts("29\n");
    console.puts("30\n");
    console.puts("31\n");
    console.puts("32\n");
    console.puts("33\n");
    console.puts("34\n");
    console.puts("35\n");
    console.puts("36\n");
    console.puts("37\n");
    console.puts("38\n");
    console.puts("39\n");
    console.puts("40\n");
    console.puts("41\n");
    console.puts("42\n");
    console.puts("43\n");
    console.puts("44\n");
    console.puts("45\n");
    console.puts("46\n");
    console.puts("47\n");
    console.puts("48\n");
    console.puts("49\n");
    console.puts("50\n");
    console.puts("51\n");
    console.puts("52\n");
    console.puts("53\n");
    console.puts("54\n");
    console.puts("55\n");
    console.puts("56\n");
    console.puts("57\n");
    console.puts("58\n");
    console.puts("59\n");

    loop {}
}

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
