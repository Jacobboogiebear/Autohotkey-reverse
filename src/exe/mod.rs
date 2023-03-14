mod valid;
mod getters;

#[derive(Debug, Clone)]
pub struct Executable {
    raw_buffer: Vec<u8>,
    nt_header_location: Option<usize>,
    arch: Option<String>,
    number_of_sections: Option<u32>,
    size_of_optional_header: Option<usize>,
    sections: Option<Vec<Section>>
}

#[derive(Debug, Clone)]
struct Section {
    title: String,
    size_of_raw_data: usize,
    pointer_to_raw_data: usize
}

impl Executable {
    pub fn new(raw_buffer: &mut Vec<u8>) -> Option<Executable> {
        let mut ret = Executable {
            raw_buffer: raw_buffer.clone(),
            nt_header_location: None,
            arch: None,
            number_of_sections: None,
            size_of_optional_header: None,
            sections: None

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
        return Some(ret);
    }
}
