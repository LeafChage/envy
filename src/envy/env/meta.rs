#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Meta {
    Encrypt,
    Encrypted,
    Comment(String),
}

impl ToString for Meta {
    fn to_string(&self) -> String {
        match self {
            Meta::Encrypt => String::from("#%ENCRYPT"),
            Meta::Encrypted => String::from("#%ENCRYPTED"),
            Meta::Comment(v) => String::from(format!("#{}", v)),
        }
    }
}
