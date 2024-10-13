use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum MmapType {
    Available = 1,
    Reserved,
    AcpiReclaimable,
    Nvs,
    Badram,
}

#[repr(C, packed)]
pub struct Mmap {
    size: u32,
    addr: u64,
    len: u64,
    typ: MmapType,
}

impl fmt::Debug for Mmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = self.addr;
        let len = self.len;
        let typ = self.typ;
        write!(
            f,
            "Mmap {{ addr: {addr:08x}, len: {len:08x}, typ: {typ:?} }}"
        )
    }
}
