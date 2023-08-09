mod macros;
mod parts;
mod validify;
use parts::{
    nt_header::NtHeader, resource_directory_table::ResourceDirectoryTable,
    section_header::SectionHeader,
};
use validify::Validify;

#[derive(Debug, Clone)]
pub struct ExeBinaryFile {
    pub bytes: Vec<u8>,
    pub offset: usize,
    pub nt_header: NtHeader,
    pub section_headers: Vec<SectionHeader>,
    pub resource_directory_table: Option<ResourceDirectoryTable>,
}

pub type Offset = usize;
pub type WithOffset<T> = (T, Offset);

impl ExeBinaryFile {
    pub fn new(file: impl ToString) -> Option<Self> {
        use std::io::Read;
        let file = file.to_string();
        let mut file = std::fs::File::open(file).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        return Self::new_from_vec(data);
    }

    pub fn new_from_vec(data: Vec<u8>) -> Option<Self> {
        let mut item = Self {
            bytes: data,
            offset: 0,
            nt_header: NtHeader::new(),
            section_headers: Vec::new(),
            resource_directory_table: None,
        };
        if !Validify::msdos_header(&item) {
            return None;
        }
        if !Validify::nt_header(&item) {
            return None;
        }
        NtHeader::conjoin(&mut item);
        for _ in 0..item.nt_header.number_of_sections {
            SectionHeader::conjoin(&mut item);
        }
        ResourceDirectoryTable::conjoin(&mut item);
        return Some(item);
    }
}

impl std::fmt::Display for ExeBinaryFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("--- ExeBinaryFile ---\r\n").unwrap();
        f.write_str(format!("Offset: {}\r\n", self.offset).as_str())
            .unwrap();
        f.write_str(format!("NT Header: {:#?}\r\n", self.nt_header).as_str())
            .unwrap();
        f.write_str(format!("Section Headers: {:#?}\r\n", self.section_headers).as_str())
            .unwrap();
        f.write_str(
            format!(
                "Resource Directory Table: {:#?}\r\n",
                self.resource_directory_table
            )
            .as_str(),
        )
        .unwrap();
        f.write_str("--- ExeBinaryFile ---\r\n").unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ExeBinaryFile;

    #[test]
    fn new_binary_exe_file() {
        ExeBinaryFile::new("./res/Demo.exe");
    }

    #[test]
    fn new_binary_exe_file_from_vec() {
        let data = include_bytes!("../res/Demo.exe").to_vec();
        ExeBinaryFile::new_from_vec(data);
    }
}
