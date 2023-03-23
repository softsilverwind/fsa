use std::collections::HashMap;

use crate::{DFA, NFA, nfa, State};

pub fn reverse(dfa: &DFA) -> NFA
{
    let mut prev: nfa::NextElems = nfa::NextElems::new();
    let max_state: State = (dfa.next.len() - 1).into();

    prev.resize_with(dfa.next.len(), || HashMap::new());

    for (state, dict) in dfa.next.iter() {
        for (&symbol, &nextstate) in dict.iter() {
            prev[State(max_state.0 - nextstate.0)].entry(symbol).or_insert(Vec::new()).push(State(max_state.0 - state.0))
        }
    }

    NFA { next: prev }
}
