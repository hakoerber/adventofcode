use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    Int(usize),
    String(String),
    Empty,
}

impl Output {
    pub fn empty() -> Self {
        Self::Empty
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(u) => Ok(write!(f, "{u}")?),
            Self::String(s) => Ok(write!(f, "{s}")?),
            Self::Empty => Ok(()),
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
