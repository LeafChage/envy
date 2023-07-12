#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env {
    key: String,
    value: String,
}

impl Env {
    pub fn new<T: Into<String>, T2: Into<String>>(key: T, value: T2) -> Self {
        Env {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn tuple(&self) -> (String, String) {
        (self.key.clone(), self.value.clone())
    }
}

impl ToString for Env {
    fn to_string(&self) -> String {
        format!("{}={}", self.key(), self.value())
    }
}
