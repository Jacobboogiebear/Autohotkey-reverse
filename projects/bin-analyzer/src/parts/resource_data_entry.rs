#[derive(Debug, Clone)]
pub struct ResourceDataEntry {
    pub data_rva: u32,
    pub data_size: u32,
    pub _data: Vec<u8>,
}

impl ResourceDataEntry {
    pub fn new() -> Self {
        Self {
            data_rva: 0,
            data_size: 0,
            _data: Vec::new(),
        }
    }
}
