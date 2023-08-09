pub mod ahk_script;
pub mod version;

use ahk_script::AhkScript;
use bin_analyzer::ExeBinaryFile;

pub struct Extractor;

impl Extractor {
    pub fn default(data: impl ToString) -> Option<AhkScript> {
        let data = data.to_string();
        let bin_file = ExeBinaryFile::new(data)?;
        let rsrc_table = {
            if bin_file.resource_directory_table.is_some() {
                bin_file.resource_directory_table.unwrap()
            } else {
                return None;
            }
        };
        for i in 0..rsrc_table.number_of_id_entries {
            if rsrc_table.id_entries[i as usize].int_id == 10 {
                let entry = rsrc_table.id_entries[i as usize]
                    ._resource_directory_table
                    .clone()?;
                let entry = entry.id_entries[0]._resource_directory_table.clone()?;
                let entry = entry.id_entries[0]._resource_data_entry.clone()?;
                let source = String::from_utf8(entry._data).unwrap();
                return AhkScript::parse(source);
            }
        }
        return None;
    }

    pub fn default_from_vec(data: Vec<u8>) -> Option<AhkScript> {
        let bin_file = ExeBinaryFile::new_from_vec(data)?;
        let rsrc_table = bin_file.resource_directory_table?;
        for i in 0..rsrc_table.number_of_id_entries {
            if rsrc_table.id_entries[i as usize].int_id == 10 {
                let entry = rsrc_table.id_entries[i as usize]
                    ._resource_directory_table
                    .clone()?;
                let entry = entry.id_entries[0]._resource_directory_table.clone()?;
                let entry = entry.id_entries[0]._resource_data_entry.clone()?;
                let source = String::from_utf8(entry._data).unwrap();
                return AhkScript::parse(source);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    use crate::Extractor;

    #[test]
    fn testing_default_extractor() {
        Extractor::default("./res/Demo.exe");
    }

    #[test]
    fn testing_default_extractor_from_vec() {
        let data = include_bytes!("../res/Demo.exe").to_vec();
        Extractor::default_from_vec(data);
    }

    #[test]
    fn testing_getting_source() {
        let script = Extractor::default("./res/Demo.exe").unwrap();
        println!("{}", script.obtain_source());
    }
}
