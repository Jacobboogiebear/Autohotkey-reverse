use crate::macros::to_exact;
use crate::ExeBinaryFile;

pub struct Validify;

impl Validify {
    pub fn msdos_header(item: &ExeBinaryFile) -> bool {
        return &item.bytes[0..2] == &[0x4D, 0x5A];
    }

    pub fn nt_header(item: &ExeBinaryFile) -> bool {
        let addr = u32::from_le_bytes(to_exact!(&item.bytes[60..64]; 4)) as usize;
        return &item.bytes[addr..addr + 4] == &[0x50, 0x45, 0x00, 0x00];
    }

    pub fn opt_header(item: &ExeBinaryFile) -> bool {
        let addr = item.offset;
        return &item.bytes[addr..addr + 2] == &[0x0B, 0x02];
    }
}
