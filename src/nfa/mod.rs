use std::{
    collections::{BTreeSet, HashMap, HashSet},
    error::Error,
    iter,
    str::FromStr,
};

use crate::{State, Symbol};

use nicole::typedvec::TypedVec;

mod regex_parser;

pub type NextElem = HashMap<Symbol, BTreeSet<State>>;
pub type NextElems = TypedVec<State, NextElem>;

#[derive(Clone, Debug)]
pub struct NFA {
    pub next: NextElems,
    pub initials: HashSet<State>,
    pub finals: HashSet<State>,
}

impl FromStr for NFA {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let next = regex_parser::parse(s)?;
        let initials = iter::once(State(0)).collect();
        let finals = iter::once(State::from(next.len().wrapping_sub(1))).collect();

        Ok(Self {
            next,
            initials,
            finals,
        })
    }
}

impl NFA {
    pub fn print_graphviz(&self) {
        indoc::printdoc!(
            "
            digraph finite_state_machine {{
                rankdir=LR;
                size=\"8,5\"
                node [shape = doublecircle]; q{terminal_id};
                node [shape = circle];
        ",
            terminal_id = self.next.len() - 1
        );

        println!();

        for state_id in 0..self.next.len() {
            println!(
                "    q{state} [ label=<q<SUB>{state}</SUB>> ];",
                state = state_id
            );
        }

        for (state, dict) in self.next.iter() {
            for (symbol, next_states) in dict.iter() {
                for next_state in next_states {
                    println!(
                        "    {state} -> {next_state} [ label={symbol} ];",
                        state = state,
                        next_state = next_state,
                        symbol = symbol
                    );
                }
            }
        }
        println!("}}");
    }
}
