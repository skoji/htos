#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *mut u8, n: isize) {
    for i in 0..n {
        *dest.offset(i) = *src.offset(i);
    }
}

#[no_mangle]
pub unsafe extern "C" fn memset(buf: *mut u8, ch: u8, n: isize) {
    for i in 0..n {
        *buf.offset(i) = ch;
    }
}
