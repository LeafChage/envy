#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Meta {
    Secret,
    Comment(String),
}

impl ToString for Meta {
    fn to_string(&self) -> String {
        match self {
            Meta::Secret => String::from("#%SECRET"),
            Meta::Comment(v) => String::from(format!("#{}", v)),
        }
    }
}
