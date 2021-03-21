#![no_main]
#![no_std]

use core::panic::PanicInfo;

extern crate htos;
use htos::boot_info::{BootInfo, Pixel};

#[link_section = ".text.entry"]
#[no_mangle]
pub unsafe extern "C" fn kernel_entry(binfo: *mut BootInfo) -> ! {
    for i in 0..(*binfo).vinfo.frame_buffer_size {
        let frame_offset: *mut Pixel = (*binfo).vinfo.frame_buffer_base.offset(i as isize);
        let pixel = Pixel {
            rgb: [0x00, 0xff, 0x00],
            _rsvd: 0,
        };
        *frame_offset = pixel;
    }
    loop {}
}

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
