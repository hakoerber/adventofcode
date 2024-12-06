use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    Int(usize),
    String(String),
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Output::Int(u) => write!(f, "{}", u),
            Output::String(s) => write!(f, "{}", s),
        }
    }
}

impl From<usize> for Output {
    fn from(value: usize) -> Self {
        Self::Int(value)
    }
}

impl From<String> for Output {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
