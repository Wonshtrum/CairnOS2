#[macro_export]
macro_rules! const_assert {
    (@size $T:ident == $v:expr) => {
        $crate::const_assert_eq!(core::mem::size_of::<$T>(), $v);
    };
    (@align $T:ident == $v:expr) => {
        $crate::const_assert_eq!(core::mem::align_of::<$T>(), $v);
    };
    (@offset $T:ty[$field:ident] == $v:expr) => {
        $crate::const_assert_eq!(core::mem::offset_of!($T, $field), $v);
    };
    ($cond:expr) => {
        const _: () = if !$cond {
            panic!()
        };
    };
}

#[macro_export]
macro_rules! const_assert_eq {
    ($actual:expr, $expected:expr) => {
        const _: [(); $expected as usize] = [(); $actual as usize];
    };
}
