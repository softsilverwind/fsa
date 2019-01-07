use std::cmp::min;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

use super::{DFANext, DFANextView, DFANextElem, StateId, SymbolId};

pub fn find_same(dfa: &DFANextView, finals: &HashSet<StateId>) -> Vec<Vec<StateId>>
{
    let statenum = dfa.len() as StateId;
    let mut different: HashSet<(StateId, StateId)> = HashSet::new();

    for i in 0..statenum {
        for j in (i+1)..statenum {
            if finals.contains(&i) != finals.contains(&j) {
                different.insert((i, j));
                different.insert((j, i));
            }
        }
    }

    let mut fixpoint = false;
    while !fixpoint {
        fixpoint = true;
        for i in 0..statenum {
            'jloop: for j in (i+1)..statenum {
                if different.contains(&(i, j)) {
                    continue;
                }

                let symbols1: HashSet<SymbolId> = HashSet::from_iter(dfa[i as usize].keys().cloned());
                let symbols2: HashSet<SymbolId> = HashSet::from_iter(dfa[j as usize].keys().cloned());

                if symbols1.symmetric_difference(&symbols2).next() != None {
                    different.insert((i, j));
                    different.insert((j, i));
                    fixpoint = false;
                    continue;
                }

                for symbol in dfa[i as usize].keys() {
                    let next1 = dfa[i as usize][&symbol];
                    let next2 = dfa[j as usize][&symbol];

                    if different.contains(&(next1, next2)) {
                        different.insert((i, j));
                        different.insert((j, i));
                        fixpoint = false;
                        continue 'jloop;
                    }
                }
            }
        }
    }

    let mut ret = vec![Vec::new(); statenum as usize];
    for i in 0..statenum {
        for j in 0..statenum {
            if !different.contains(&(i, j)) {
                ret[i as usize].push(j);
            }
        }
        ret[i as usize].sort();
    }

    ret
}

pub fn minimize_dfa(dfa: &mut DFANext, finals: &mut HashSet<StateId>)
{
    let same = find_same(dfa, finals);

    let mut new_dfa = DFANext::new();
    let mut translate: HashMap<StateId, StateId> = HashMap::new();
    let mut curr_state_id = 0;

    for (state_id, state) in dfa.iter().enumerate() {
        let state_id = state_id as StateId;
        if *same[state_id as usize].get(0).unwrap_or_else(|| &state_id) < state_id {
            continue;
        }

        translate.insert(state_id, curr_state_id);
        curr_state_id += 1;

        let mut new_state = DFANextElem::new();
        for (symbol, next) in state {
            let untranslated_actual_next = match same[*next as usize].get(0) {
                Some(x) => min(x, next),
                None => next
            };
            new_state.insert(*symbol, *untranslated_actual_next);
        }
        new_dfa.push(new_state);
    }

    for state in new_dfa.iter_mut() {
        for (_, next) in state.iter_mut() {
            *next = *translate.get(next).unwrap_or_else(|| next);
        }
    }

    let mut new_finals: HashSet<StateId> = HashSet::new();
    for f in finals.iter() {
        if let Some(x) = translate.get(f) {
            new_finals.insert(*x);
        }
    }

    *dfa = new_dfa;
    *finals = new_finals;
}
