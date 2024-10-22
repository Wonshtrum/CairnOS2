pub mod gdt;

#[derive(Debug)]
#[repr(C, packed(2))]
struct Descriptor {
    size: u16,
    offset: u32,
}
const_assert!(@size Descriptor == 6);
