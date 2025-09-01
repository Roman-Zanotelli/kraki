use std::fmt::Display;

use crate::symbol::Symbol;

pub struct Pair(Symbol, Symbol);

impl Display for Pair{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}/{}", self.0, self.1))
    }
}