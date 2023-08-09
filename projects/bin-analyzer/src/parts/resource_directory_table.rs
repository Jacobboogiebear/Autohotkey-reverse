use crate::{
    macros::{iwo, to_exact},
    ExeBinaryFile,
};

use super::{
    id_entry::IdEntry, resource_data_entry::ResourceDataEntry, section_header::SectionHeader,
};

#[derive(Debug, Clone)]
pub struct ResourceDirectoryTable {
    pub number_of_id_entries: u16,
    pub id_entries: Vec<IdEntry>,
    pub _depth: u8,
}

impl ResourceDirectoryTable {
    pub fn new() -> Self {
        ResourceDirectoryTable {
            number_of_id_entries: 0,
            id_entries: Vec::new(),
            _depth: 0,
        }
    }

    pub fn new_by_depth(depth: u8) -> Self {
        ResourceDirectoryTable {
            number_of_id_entries: 0,
            id_entries: Vec::new(),
            _depth: depth,
        }
    }

    pub fn parse(&mut self, val: &mut ExeBinaryFile, rsrc: u32, root: u32) {
        let mut offset = rsrc + 14;
        iwo!(offset, 2; {
            self.number_of_id_entries = u16::from_le_bytes(to_exact!(&val.bytes[offset as usize..offset as usize + 2]; 2));
        });
        let mut rsrc_header = SectionHeader::new();
        for i in 0..val.section_headers.len() {
            if val.section_headers[i].name == ".rsrc" {
                rsrc_header = val.section_headers[i].clone();
            }
        }
        for _ in 0..self.number_of_id_entries {
            let mut entry = IdEntry::new();
            iwo!(offset, 4; {
                entry.int_id = u32::from_le_bytes(to_exact!(&val.bytes[offset as usize..offset as usize + 4]; 4));
            });
            if self._depth < 2 {
                iwo!(offset, 4; {
                    entry.subdir_offset = Some(u32::from_le_bytes(to_exact!(&val.bytes[offset as usize..offset as usize + 4]; 4)));
                });
                entry._subdir_addr = Some(
                    u16::from_le_bytes(to_exact!(&entry.subdir_offset.unwrap().to_le_bytes(); 2))
                        as u32
                        + root,
                );
                let mut new_table = ResourceDirectoryTable::new_by_depth(self._depth + 1);
                new_table.parse(val, entry._subdir_addr.unwrap(), root);
                entry._resource_directory_table = Some(new_table);
            } else {
                iwo!(offset, 4; {
                    entry.data_entry_offset = Some(u32::from_le_bytes(to_exact!(&val.bytes[offset as usize..offset as usize + 4]; 4)));
                });
                let mut data = ResourceDataEntry::new();
                let mut roffset = root + entry.data_entry_offset.clone().unwrap();
                iwo!(roffset, 4; {
                    data.data_rva = u32::from_le_bytes(to_exact!(&val.bytes[roffset as usize..roffset as usize + 4]; 4));
                });
                iwo!(roffset, 4; {
                    data.data_size = u32::from_le_bytes(to_exact!(&val.bytes[roffset as usize..roffset as usize + 4]; 4));
                });
                let data_addr_root =
                    data.data_rva - rsrc_header.virtual_address + rsrc_header.pointer_to_raw_data;
                let raw_data = &val.bytes
                    [data_addr_root as usize..data_addr_root as usize + data.data_size as usize];
                data._data = raw_data.to_vec();
                entry._resource_data_entry = Some(data);
            }
            self.id_entries.push(entry);
        }
    }

    pub fn conjoin(val: &mut ExeBinaryFile) {
        let mut item = Self::new();
        let mut rsrc: Option<u32> = None;
        for i in 0..val.section_headers.len() {
            if val.section_headers[i].name == ".rsrc" {
                rsrc = Some(val.section_headers[i].pointer_to_raw_data);
                break;
            }
        }
        if rsrc.is_some() {
            let rsrc = rsrc.unwrap();
            item.parse(val, rsrc, rsrc);
            val.resource_directory_table = Some(item);
        }
    }
}
