#[derive(Debug, Clone)]
pub struct OptionalHeader {
    pub number_of_rva_and_sizes: u32,
}

impl OptionalHeader {
    pub fn new() -> Self {
        OptionalHeader {
            number_of_rva_and_sizes: 0,
        }
    }
}
