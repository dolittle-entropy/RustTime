use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    build: u8,
    pre_release_string: String,
}

impl Version {
    pub fn not_set() -> Version {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
            build: 0,
            pre_release_string: "".to_string(),
        }
    }

    fn is_pre_release(&self) -> bool {
        !self.pre_release_string.is_empty()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut displayed_string = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if self.is_pre_release() {
            displayed_string
                .push_str(format!("-{}.{}", self.pre_release_string, self.build).as_str());
        }
        f.write_str(displayed_string.as_str())?;
        Ok(())
    }
}
