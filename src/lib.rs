#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};

mod regex_to_nfa;
mod nfa_to_dfa;
mod dfa_minimization;

use self::regex_to_nfa::regex_to_nfa;
use self::nfa_to_dfa::nfa_to_dfa;
use self::dfa_minimization::minimize_dfa;

pub type StateId = i32;
pub type SymbolId = i32;

type NFANextElem = HashMap<SymbolId, Vec<StateId>>;
type DFANextElem = HashMap<SymbolId, StateId>;

type NFANext = Vec<NFANextElem>;
type DFANext = Vec<DFANextElem>;

#[derive(Debug)]
pub struct NFA
{
    next: NFANext
}

#[derive(Debug)]
pub struct DFA
{
    next: DFANext,
    finals: HashSet<StateId>
}

impl NFA
{
    pub fn from_regex(regex: &str) -> Result<Self, String>
    {
        Ok(Self { next: regex_to_nfa(regex)? })
    }

    pub fn next(&self, symbol: SymbolId, state: StateId) -> &Vec<StateId>
    {
        &self.next[state as usize][&symbol]
    }

    pub fn print_graphviz(&self)
    {
        println!("
            digraph finite_state_machine {{
                rankdir=LR;
                size=\"8,5\"
                node [shape = doublecircle]; {terminal};
                node [shape = circle];
        ", terminal = self.next.len() - 1);

        for (state, dict) in self.next.iter().enumerate() {
            for (symbol, next_states) in dict {
                for next_state in next_states {
                    println!("{state} -> {next_state} [ label = \"{symbol}\" ];",
                             state=state, next_state=next_state, symbol=symbol);
                }
            }
        }
        println!("}}");
    }
}

impl DFA
{
    pub fn from_nfa(nfa: &NFA) -> Self
    {
        let (next, finals) = nfa_to_dfa(&nfa.next);
        Self { next, finals }
    }

    pub fn next(&self, symbol: SymbolId, state: StateId) -> StateId
    {
        self.next[state as usize][&symbol]
    }

    pub fn is_final(&self, state: StateId) -> bool
    {
        self.finals.contains(&state)
    }
    
    pub fn minimize(&mut self)
    {
        minimize_dfa(&mut self.next, &mut self.finals);
    }

    pub fn print_graphviz(&self)
    {
        println!("
            digraph finite_state_machine {{
                rankdir=LR;
                size=\"8,5\"
        ");

        for x in self.finals.iter() {
            println!("
                    node [shape = doublecircle]; {terminal};
            ", terminal = x);
        }
        println!("node [shape = circle];");

        for (state, dict) in self.next.iter().enumerate() {
            for (symbol, next_state) in dict {
                println!("{state} -> {next_state} [ label = \"{symbol}\" ];",
                            state=state, next_state=next_state, symbol=symbol);
            }
        }
        println!("}}");
    }
}