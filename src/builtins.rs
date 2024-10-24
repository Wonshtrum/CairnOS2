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

#[no_mangle]
pub unsafe extern "C" fn memcmp(ptr1: *const u8, ptr2: *const u8, num: usize) -> isize {
    for i in 0..num {
        let a = *ptr1.add(i);
        let b = *ptr2.add(i);
        if a != b {
            return a as isize - b as isize;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strlen(src: *const u8) -> usize {
    let mut i = 0;
    while *src.add(i) != 0 {
        i += 1;
    }
    i
}
