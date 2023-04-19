use std::collections::{HashMap, HashSet};

use crate::{nfa::NFA, State, Symbol};

use nicole::typedvec::TypedVec;

mod convert;
mod generate;
mod minimize;
mod reverse;

pub type NextElem = HashMap<Symbol, State>;
pub type NextElems = TypedVec<State, NextElem>;

#[derive(Clone, Debug)]
pub struct DFA {
    pub next: NextElems,
    pub initial: State,
    pub finals: HashSet<State>,
}

impl From<NFA> for DFA {
    fn from(nfa: NFA) -> Self {
        convert::nfa_to_dfa(nfa)
    }
}

impl DFA {
    pub fn minimize(mut self) -> Self {
        minimize::minimize_dfa(&mut self.next, &mut self.finals);
        self
    }

    pub fn print_graphviz(&self) {
        indoc::printdoc!(
            r#"
            digraph finite_state_machine {{
                rankdir=LR;
                size="8,5"
        "#
        );

        for x in self.finals.iter() {
            println!("    node [shape = doublecircle]; {terminal};", terminal = x);
        }
        println!("    node [shape = circle];");

        for state_id in 0..self.next.len() {
            println!(
                "    q{state} [ label=<q<SUB>{state}</SUB>> ];",
                state = state_id
            );
        }

        for (state, dict) in self.next.iter() {
            for (symbol, next_state) in dict.iter() {
                println!(
                    "    {state} -> {next_state} [ label={symbol} ];",
                    state = state,
                    next_state = next_state,
                    symbol = symbol
                );
            }
        }
        println!("}}");
    }

    pub fn matches(&self, string: &[Symbol]) -> bool {
        let mut state = self.initial;

        for symbol in string {
            match self.next[state].get(symbol) {
                Some(&next_state) => state = next_state,
                None => return false,
            }
        }

        self.finals.contains(&state)
    }

    pub fn reverse(&self) -> NFA {
        reverse::reverse(self)
    }
}
