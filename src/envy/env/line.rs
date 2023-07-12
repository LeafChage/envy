use super::{Env, Meta};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Meta(Meta),
    Env(Env),
}
