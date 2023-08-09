use super::{
    resource_data_entry::ResourceDataEntry, resource_directory_table::ResourceDirectoryTable,
};

#[derive(Debug, Clone)]
pub struct IdEntry {
    pub int_id: u32,
    pub subdir_offset: Option<u32>,
    pub data_entry_offset: Option<u32>,
    pub _subdir_addr: Option<u32>,
    pub _resource_directory_table: Option<ResourceDirectoryTable>,
    pub _resource_data_entry: Option<ResourceDataEntry>,
}

impl IdEntry {
    pub fn new() -> Self {
        Self {
            int_id: 0,
            subdir_offset: None,
            data_entry_offset: None,
            _subdir_addr: None,
            _resource_directory_table: None,
            _resource_data_entry: None,
        }
    }
}
