#[path = "../bin_utils.rs"]
mod bin_utils;

mod getters;
mod parser;
mod valid;

#[derive(Debug, Clone)]
pub struct Executable {
    raw_buffer: Vec<u8>,
    nt_header_location: Option<usize>,
    arch: Option<String>,
    number_of_sections: Option<u32>,
    size_of_optional_header: Option<usize>,
    sections: Option<Vec<Section>>,
    rsrc_section: Option<Vec<u8>>,
    ahk_version: Option<String>,
    compiler_version: Option<String>,
}

#[derive(Debug, Clone)]
struct Section {
    title: String,
    size_of_raw_data: usize,
    pointer_to_raw_data: usize,
}

impl Executable {
    pub fn new(raw_buffer: &mut Vec<u8>) -> Option<Executable> {
        let mut ret = Executable {
            raw_buffer: raw_buffer.clone(),
            nt_header_location: None,
            arch: None,
            number_of_sections: None,
            size_of_optional_header: None,
            sections: None,
            rsrc_section: None,
            ahk_version: None,
            compiler_version: None,
        };
        if !ret.is_valid_dos() {
            return None;
        }
        ret.get_nt_header_location();
        if !ret.is_valid_nt_header() {
            return None;
        }
        ret.get_architecture();
        if ret.arch.is_none() || ret.arch.clone().unwrap() == String::from("Unknown") {
            return None;
        }
        ret.get_number_of_sections();
        ret.get_size_of_optional_header();
        ret.get_sections();
        ret.get_rsrc_section();
        ret.parse_ahk_version();
        ret.parse_ahk_compiler();
        return Some(ret);
    }

    pub fn get_source(&mut self) -> String {
        let end_of_source: Vec<u8> = vec![0x00, 0x00, 0x01];
        let source_start: usize = {
            let compiler_check: Vec<u8> = vec![
                0x3C, 0x43, 0x4F, 0x4D, 0x50, 0x49, 0x4C, 0x45, 0x52, 0x3A, 0x20, 0x76,
            ];
            let compiler_location = bin_utils::vec_slide_search(
                &mut self.rsrc_section.clone().unwrap(),
                &compiler_check,
            );
            let compiler_version = bin_utils::vec_get_string_ct(
                &mut self.rsrc_section.clone().unwrap(),
                compiler_location.unwrap() + compiler_check.len(),
                char::from(0x3E),
            );
            compiler_location.unwrap() as usize
                + compiler_check.len()
                + compiler_version.unwrap().len()
                + 0x2
        };
        let untrimmed_source = bin_utils::vec_get_string_st(
            &mut self.rsrc_section.clone().unwrap(),
            source_start,
            String::from_utf8(end_of_source).unwrap(),
        )
        .unwrap();
        let mut source = String::new();
        if untrimmed_source.ends_with("P") {
            source = untrimmed_source[..untrimmed_source.len() - 1].to_string();
        } else if untrimmed_source.ends_with("A") {
            source = untrimmed_source[..untrimmed_source.len() - 2].to_string();
        } else if untrimmed_source.ends_with("D") {
            source = untrimmed_source[..untrimmed_source.len() - 3].to_string();
        } else {
            return untrimmed_source;
        }
        return source;
    }
}
