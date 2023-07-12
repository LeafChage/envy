use super::{Env, Meta};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Meta(Meta),
    Env(Env),
}

impl ToString for Line {
    fn to_string(&self) -> String {
        match self {
            Self::Meta(v) => v.to_string(),
            Self::Env(v) => v.to_string(),
        }
    }
}
