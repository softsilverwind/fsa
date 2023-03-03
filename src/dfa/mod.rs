use std::collections::{HashSet, HashMap};

use crate::{Symbol, State, nfa::NFA};

mod convert;
mod generate;
mod minimize;

pub type NextElem = HashMap<Symbol, State>;
pub type NextElems = Vec<NextElem>;
pub type NextElemsView = [NextElem];

#[derive(Clone, Debug)]
pub struct DFA
{
    pub next: NextElems,
    pub finals: HashSet<State>
}

impl From<&NFA> for DFA
{
    fn from(nfa: &NFA) -> Self
    {
        let (next, finals) = convert::nfa_to_dfa(&nfa.next);
        Self { next, finals }
    }
}

impl DFA
{
    pub fn minimize(&mut self)
    {
        minimize::minimize_dfa(&mut self.next, &mut self.finals);
    }

    pub fn print_graphviz(&self)
    {
        indoc::printdoc!(r#"
            digraph finite_state_machine {{
                rankdir=LR;
                size="8,5"
        "#);

        for x in self.finals.iter() {
            println!("    node [shape = doublecircle]; {terminal};", terminal = x);
        }
        println!("    node [shape = circle];");

        for state_id in 0..self.next.len() {
            println!(
                "    q{state} [ label=<q<SUB>{state}</SUB>> ];",
                state=state_id
            );
        }

        for (state_id, dict) in self.next.iter().enumerate() {
            let state: State = state_id.into();
            for (symbol, next_state) in dict {
                println!(
                    "    {state} -> {next_state} [ label={symbol} ];",
                    state=state,
                    next_state=next_state,
                    symbol=symbol
                );
            }
        }
        println!("}}");
    }

    pub fn matches(&self, string: &[Symbol]) -> bool
    {
        let mut state = State(0);

        for symbol in string {
            match self.next[state.0 as usize].get(&symbol) {
                Some(&next_state) => state = next_state,
                None => return false
            }
        }

        self.finals.contains(&state)
    }
}
