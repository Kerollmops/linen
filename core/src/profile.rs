#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Profile {
    Full,
    Embedded,
}

impl<'a> From<&'a str> for Profile {
    fn from(value: &'a str) -> Self {
        match value {
            "FULL_PROFILE" => Profile::Full,
            "EMBEDDED_PROFILE" => Profile::Embedded,
            _ => panic!("Unknown profile type")
        }
    }
}
