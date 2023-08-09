use crate::version::Version;

#[derive(Debug, Clone)]
pub struct AhkScript {
    pub version: Version,
    pub source: String,
}

impl AhkScript {
    pub fn parse(source: String) -> Option<Self> {
        if source.starts_with("; <COMPILER: v") {
            let version_str = *(&source["; <COMPILER: v".len()..]
                .split(">")
                .collect::<Vec<&str>>()[0]);
            let version = Version::parse(version_str)?;
            let source_code = source.split('\x0A').collect::<Vec<&str>>()[1..].join("\x0A");
            return Some(Self {
                version,
                source: source_code,
            });
        } else {
            return None;
        }
    }

    pub fn obtain_source(&self) -> String {
        return format!("; Compiled with AHK Version {}\r\n; Decompiled using Autohotkey-reverse from Github at https://github.com/Jacobboogiebear/Autohotkey-reverse\r\n{}", self.version.to_string(), self.source);
    }
}
