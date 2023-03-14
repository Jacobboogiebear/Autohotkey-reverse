#[path = "../bin_utils.rs"]
mod bin_utils;

use super::Executable;

impl Executable {
    pub(super) fn is_valid_dos(&mut self) -> bool {
        return bin_utils::compare_bytes(&mut self.raw_buffer, vec![0x4D, 0x5A], 0);
    }

    pub(super) fn get_nt_header_location(&mut self) {
        let new_loc = bin_utils::vec_get_u32(&mut self.raw_buffer, 0x3C).unwrap() as usize;
        self.nt_header_location = Some(new_loc);
    }

    pub(super) fn is_valid_nt_header(&mut self) -> bool {
        return bin_utils::compare_bytes(
            &mut self.raw_buffer,
            vec![0x50, 0x45, 0x00, 0x00],
            self.nt_header_location.unwrap(),
        );
    }
}
