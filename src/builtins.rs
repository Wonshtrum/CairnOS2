#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, val: i32, num: usize) -> *mut u8 {
    let val = val as u8;
    for i in 0..num {
        *ptr.add(i) = val;
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, num: usize) -> *mut u8 {
    for i in 0..num {
        *dst.add(i) = *src.add(i);
    }
    dst
}
