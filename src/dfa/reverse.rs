use std::{collections::HashMap, iter};

use crate::{nfa, DFA, NFA};

pub fn reverse(dfa: &DFA) -> NFA {
    let mut prev: nfa::NextElems = nfa::NextElems::new();
    prev.resize_with(dfa.next.len(), || HashMap::new());

    for (state, dict) in dfa.next.iter() {
        for (&symbol, &nextstate) in dict.iter() {
            prev[nextstate].entry(symbol).or_default().insert(state);
        }
    }

    NFA {
        next: prev,
        initials: dfa.finals.clone(),
        finals: iter::once(dfa.initial).collect(),
    }
}
