#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Meta {
    Encrypt,
    Encrypted(String),
    Comment(String),
    WhiteSpaces,
}

impl ToString for Meta {
    fn to_string(&self) -> String {
        match self {
            Meta::Encrypt => String::from("#%ENCRYPT"),
            Meta::Encrypted(v) => String::from(format!("#%ENCRYPTED({})", v)),
            Meta::Comment(v) => String::from(format!("#{}", v)),
            Meta::WhiteSpaces => String::from(""),
        }
    }
}
