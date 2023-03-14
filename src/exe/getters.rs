#[path = "../bin_utils.rs"]
mod bin_utils;

use super::{Executable, Section};

impl Executable {
    pub(super) fn get_architecture(&mut self) {
        if bin_utils::compare_bytes(
            &mut self.raw_buffer,
            vec![0x64, 0x86],
            &self.nt_header_location.unwrap() + 0x4,
        ) {
            self.arch = Some(String::from("x86_64"));
        } else if bin_utils::compare_bytes(
            &mut self.raw_buffer,
            vec![0x4C, 0x01],
            &self.nt_header_location.unwrap() + 0x4,
        ) {
            self.arch = Some(String::from("i386"));
        } else {
            self.arch = Some(String::from("Unknown"));
        }
    }

    pub(super) fn get_number_of_sections(&mut self) {
        let number_of_sections =
            bin_utils::vec_get_u16(&mut self.raw_buffer, self.nt_header_location.unwrap() + 0x6);
        if number_of_sections.is_some() {
            self.number_of_sections = Some(number_of_sections.unwrap() as u32);
        }
    }

    pub(super) fn get_size_of_optional_header(&mut self) {
        let size_of_optional_header = bin_utils::vec_get_u16(
            &mut self.raw_buffer,
            self.nt_header_location.clone().unwrap() + 0x14,
        );
        if size_of_optional_header.is_some() {
            self.size_of_optional_header = Some(size_of_optional_header.unwrap() as usize);
        }
    }

    pub(super) fn get_sections(&mut self) {
        let mut ret: Vec<Section> = vec![];
        for index in 0..self.number_of_sections.unwrap() {
            let init_loc = self.nt_header_location.clone().unwrap()
                + 0x18
                + self.size_of_optional_header.clone().unwrap()
                + index as usize * 0x28;
            let title = bin_utils::vec_get_string_nnt(&mut self.raw_buffer, init_loc, 8);
            let size_of_raw_data = bin_utils::vec_get_u32(&mut self.raw_buffer, init_loc + 0x10);
            let pointer_to_raw_data = bin_utils::vec_get_u32(&mut self.raw_buffer, init_loc + 0x14);
            ret.push(Section {
                title: title.unwrap(),
                size_of_raw_data: size_of_raw_data.unwrap() as usize,
                pointer_to_raw_data: pointer_to_raw_data.unwrap() as usize,
            });
        }
        self.sections = Some(ret);
    }
}
