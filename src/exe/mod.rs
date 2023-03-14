#[path = "../bin_utils.rs"]
mod bin_utils;

mod getters;
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
    compiler_version: Option<String>
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
            compiler_version: None
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
        return Some(ret);
    }

    pub fn parse_ahk_version(&mut self) -> String {
        // <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0" xmlns:v3="urn:schemas-microsoft-com:asm.v3">
        let compare: Vec<u8> = vec![
            0x3C, 0x61, 0x73, 0x73, 0x65, 0x6D, 0x62, 0x6C, 0x79, 0x20, 0x78, 0x6D, 0x6C, 0x6E,
            0x73, 0x3D, 0x22, 0x75, 0x72, 0x6E, 0x3A, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x73,
            0x2D, 0x6D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x2D, 0x63, 0x6F, 0x6D,
            0x3A, 0x61, 0x73, 0x6D, 0x2E, 0x76, 0x31, 0x22, 0x20, 0x6D, 0x61, 0x6E, 0x69, 0x66,
            0x65, 0x73, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E,
            0x30, 0x22, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x76, 0x33, 0x3D, 0x22, 0x75,
            0x72, 0x6E, 0x3A, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x73, 0x2D, 0x6D, 0x69, 0x63,
            0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x2D, 0x63, 0x6F, 0x6D, 0x3A, 0x61, 0x73, 0x6D,
            0x2E, 0x76, 0x33, 0x22, 0x3E,
        ];
        let manifest_location =
            bin_utils::vec_slide_search(&mut self.rsrc_section.clone().unwrap(), &compare);
        let version_start_location = bin_utils::vec_get_string_ct(
            &mut self.rsrc_section.clone().unwrap(),
            manifest_location.unwrap() + compare.len(),
            char::from(0x22),
        )
        .unwrap()
        .len()
            + 1;
        let ahk_version = bin_utils::vec_get_string_ct(
            &mut self.rsrc_section.clone().unwrap(),
            manifest_location.unwrap() + version_start_location + compare.len(),
            char::from(0x22),
        );
        self.ahk_version = Some(ahk_version.clone().unwrap());
        return ahk_version.unwrap();
    }

    pub fn parse_ahk_compiler(&mut self) -> String {
        // <COMPILER: v
        let compare: Vec<u8> = vec![
            0x3C, 0x43, 0x4F, 0x4D, 0x50, 0x49, 0x4C, 0x45, 0x52, 0x3A, 0x20, 0x76,
        ];
        let compiler_location =
            bin_utils::vec_slide_search(&mut self.rsrc_section.clone().unwrap(), &compare);
        let compiler_version = bin_utils::vec_get_string_ct(
            &mut self.rsrc_section.clone().unwrap(),
            compiler_location.unwrap() + compare.len(),
            char::from(0x3E),
        );
        self.compiler_version = Some(compiler_version.clone().unwrap());
        return compiler_version.unwrap();

    }
}
