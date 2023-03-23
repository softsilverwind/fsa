use std::{
    fmt::{Debug, Display}
};

use nicole::IdLike;
use nicole_derive::IdLike;

pub mod nfa;
pub mod dfa;

pub use crate::{
    nfa::NFA,
    dfa::DFA
};

#[derive(Default, Clone, Copy, Hash, Eq, IdLike, Ord, PartialEq, PartialOrd)] pub struct State(i32);
#[derive(Default, Clone, Copy, Hash, Eq, IdLike, Ord, PartialEq, PartialOrd)] pub struct Symbol(i32);

impl Display for State
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "q{}", self.0)
    }
}

impl Debug for State
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "q{}", self.0)
    }
}

impl Display for Symbol
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.is_null() {
            write!(f, "ε")
        }
        else if self.0 >= 0 && self.0 <= 26 {
            write!(f, "{}", (self.0 as u8 + 97) as char)
        }
        else {
            write!(f, "{}", self.0)
        }
    }
}

impl Debug for Symbol
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.is_null() {
            write!(f, "ε")
        }
        else if self.0 >= 0 && self.0 <= 26 {
            write!(f, "{}", (self.0 as u8 + 97) as char)
        }
        else {
            write!(f, "{}", self.0)
        }
    }
}
