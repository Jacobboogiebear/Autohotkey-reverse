#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub extra: u32,
}

impl Version {
    pub fn parse(data: impl ToString) -> Option<Self> {
        let data = data.to_string();
        let sdata = data.split(".").collect::<Vec<&str>>();
        if sdata.len() == 4 {
            let major = u32::from_str_radix(sdata[0], 10);
            if major.is_err() {
                return None;
            }
            let major = major.unwrap();
            let minor = u32::from_str_radix(sdata[1], 10);
            if minor.is_err() {
                return None;
            }
            let minor = minor.unwrap();
            let patch = u32::from_str_radix(sdata[2], 10);
            if patch.is_err() {
                return None;
            }
            let patch = patch.unwrap();
            let extra = u32::from_str_radix(sdata[3], 10);
            if extra.is_err() {
                return None;
            }
            let extra = extra.unwrap();
            return Some(Self {
                major,
                minor,
                patch,
                extra,
            });
        }
        return None;
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{}.{}.{}.{}",
            self.major, self.major, self.patch, self.extra
        );
    }
}
