use std::{fmt::Display, ops::Deref};

pub struct Symbol(String);

impl Deref for Symbol{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for Symbol{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}