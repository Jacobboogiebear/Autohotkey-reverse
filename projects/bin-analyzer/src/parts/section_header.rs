use crate::{macros::{iwo, to_exact}, ExeBinaryFile};

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: String,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
}

impl SectionHeader {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            virtual_address: 0,
            size_of_raw_data: 0,
            pointer_to_raw_data: 0,
        }
    }

    pub fn conjoin(val: &mut ExeBinaryFile) {
        let mut offset = val.offset;
        let mut item = Self::new();
        iwo!(offset, 12; {
            let mut name = String::new();
            let name_bytes = &val.bytes[offset..offset + 8];
            for i in 0..8 {
                if name_bytes[i] >= 32 && name_bytes[i] <= 127 {
                    name.push(name_bytes[i] as char);
                }
            }
            item.name = name;
        });
        iwo!(offset, 4; {
            item.virtual_address = u32::from_le_bytes(to_exact!(&val.bytes[offset..offset + 4]; 4));
        });
        iwo!(offset, 4; {
            item.size_of_raw_data = u32::from_le_bytes(to_exact!(&val.bytes[offset..offset + 4]; 4));
        });
        iwo!(offset, 20; {
            item.pointer_to_raw_data = u32::from_le_bytes(to_exact!(&val.bytes[offset..offset + 4]; 4));
        });
        val.section_headers.push(item);
        val.offset = offset;
    }
}
