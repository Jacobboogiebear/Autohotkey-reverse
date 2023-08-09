use crate::{
    macros::{iwo, to_exact},
    validify::Validify,
    ExeBinaryFile,
};

use super::optional_header::OptionalHeader;

#[derive(Debug, Clone)]
pub enum MachineType {
    AMD64,
    Unspecified,
}

#[derive(Debug, Clone)]
pub struct NtHeader {
    pub machine: MachineType,
    pub number_of_sections: u16,
    pub size_of_optional_header: u16,
    pub optional_header: OptionalHeader,
}

impl NtHeader {
    pub fn new() -> Self {
        NtHeader {
            machine: MachineType::Unspecified,
            number_of_sections: 0,
            size_of_optional_header: 0,
            optional_header: OptionalHeader::new(),
        }
    }

    fn get_machine_type<'a>(data: &'a [u8]) -> MachineType {
        match data {
            &[0x64, 0x86] => return MachineType::AMD64,
            _ => return MachineType::Unspecified,
        }
    }

    pub fn conjoin(val: &mut ExeBinaryFile) {
        let mut nt_header = NtHeader::new();
        let mut offset = u32::from_le_bytes(to_exact!(&val.bytes[60..64]; 4)) as usize + 4;
        iwo!(offset, 2; {
            nt_header.machine = Self::get_machine_type(&val.bytes[offset..offset + 2]);
        });
        iwo!(offset, 14; {
            nt_header.number_of_sections = u16::from_le_bytes(to_exact!(&val.bytes[offset..offset + 2]; 2));
        });
        iwo!(offset, 4; {
            nt_header.size_of_optional_header = u16::from_le_bytes(to_exact!(&val.bytes[offset..offset + 2]; 2));
        });
        let mut cl = val.clone();
        cl.offset = offset;
        cl.nt_header = nt_header.clone();
        if Validify::opt_header(&cl) {
            let mut opt_header = OptionalHeader::new();
            offset += 108;
            iwo!(offset, 4; {
                opt_header.number_of_rva_and_sizes = u32::from_le_bytes(to_exact!(&val.bytes[offset..offset + 4]; 4));
            });
            offset += opt_header.number_of_rva_and_sizes as usize * 8;
            val.offset = offset;
            val.nt_header = cl.nt_header.clone();
            val.nt_header.optional_header = opt_header.clone();
        }
    }
}
