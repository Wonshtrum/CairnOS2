#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

pub struct ProbablyInit<T> {
    val: MaybeUninit<T>,
    init: bool,
}

pub struct LazyInner<T, const MUT: bool> {
    inner: UnsafeCell<ProbablyInit<T>>,
}
pub type Lazy<T> = LazyInner<T, false>;
pub type LazyMut<T> = LazyInner<T, true>;

impl<T> ProbablyInit<T> {
    pub const fn new_uninit() -> Self {
        Self {
            val: MaybeUninit::uninit(),
            init: false,
        }
    }

    pub fn new(val: T) -> Self {
        Self {
            val: MaybeUninit::new(val),
            init: true,
        }
    }

    pub fn is_init(&self) -> bool {
        self.init
    }
}

impl<T> Deref for ProbablyInit<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.val.assume_init_ref() }
    }
}

impl<T> DerefMut for ProbablyInit<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.val.assume_init_mut() }
    }
}

unsafe impl<T, const MUT: bool> Sync for LazyInner<T, MUT> {}

impl<T, const MUT: bool> LazyInner<T, MUT> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(ProbablyInit::new_uninit()),
        }
    }

    pub unsafe fn init(&self, val: T) {
        *self.inner.get() = ProbablyInit::new(val)
    }

    pub fn is_init(&self) -> bool {
        unsafe { (*self.inner.get()).init }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    pub fn try_get(&self) -> Option<&T> {
        self.is_init().then_some(self.get())
    }
}

impl<T> LazyMut<T> {
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn try_get_mut(&self) -> Option<&mut T> {
        self.is_init().then_some(self.get_mut())
    }
}
