use std::fmt::{Debug, Display};

use nicole::IdLike;
use nicole_derive::IdLike;

pub mod dfa;
pub mod nfa;

#[cfg(test)]
mod test;

pub use crate::{dfa::DFA, nfa::NFA};

#[derive(Default, Clone, Copy, Hash, Eq, IdLike, Ord, PartialEq, PartialOrd)]
pub struct State(i32);
#[derive(Default, Clone, Copy, Hash, Eq, IdLike, Ord, PartialEq, PartialOrd)]
pub struct Symbol(i32);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "q{}", self.0)
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "q{}", self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "ε")
        } else if self.0 >= 0 && self.0 <= 26 {
            write!(f, "{}", (self.0 as u8 + 97) as char)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "ε")
        } else if self.0 >= 0 && self.0 <= 26 {
            write!(f, "{}", (self.0 as u8 + 97) as char)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Symbol {
    pub fn from_u8(c: u8) -> Self {
        Symbol((c - 97) as i32)
    }
}
