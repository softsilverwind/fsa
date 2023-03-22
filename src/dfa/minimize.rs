use std::cmp::min;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

use crate::{State, Symbol, dfa};

pub fn find_same(dfa: &dfa::NextElems, finals: &HashSet<State>) -> Vec<Vec<State>>
{
    let statenum = dfa.len().into();
    let mut different: HashSet<(State, State)> = HashSet::new();

    for i in (0..statenum).map(|x| x.into()) {
        for j in ((usize::from(i) + 1)..statenum).map(|x| x.into()) {
            if finals.contains(&i) != finals.contains(&j) {
                different.insert((i, j));
                different.insert((j, i));
            }
        }
    }

    let mut fixpoint = false;
    while !fixpoint {
        fixpoint = true;
        for i in (0..statenum).map(|x| x.into()) {
            for j in ((usize::from(i) + 1)..statenum).map(|x| x.into()) {
                if different.contains(&(i, j)) {
                    continue;
                }

                let symbols1: HashSet<Symbol> = HashSet::from_iter(dfa[i].keys().copied());
                let symbols2: HashSet<Symbol> = HashSet::from_iter(dfa[j].keys().copied());

                if symbols1.symmetric_difference(&symbols2).next() != None {
                    different.insert((i, j));
                    different.insert((j, i));
                    fixpoint = false;
                    continue;
                }

                for symbol in dfa[i].keys() {
                    let next1 = dfa[i][&symbol];
                    let next2 = dfa[j][&symbol];

                    if different.contains(&(next1, next2)) {
                        different.insert((i, j));
                        different.insert((j, i));
                        fixpoint = false;
                        break;
                    }
                }
            }
        }
    }

    let mut ret = vec![Vec::new(); statenum as usize];
    for i in (0..statenum).map(|x| x.into()) {
        for j in ((usize::from(i) + 1)..statenum).map(|x| x.into()) {
            if !different.contains(&(i, j)) {
                ret[usize::from(i)].push(j);
                ret[usize::from(j)].push(i);
            }
        }
        ret[usize::from(i)].sort();
    }

    ret
}

pub fn minimize_dfa(dfa: &mut dfa::NextElems, finals: &mut HashSet<State>)
{
    let same = find_same(dfa, finals);

    let mut new_dfa = dfa::NextElems::new();
    let mut translate: HashMap<State, State> = HashMap::new();
    let mut curr_state_id = 0;

    for (state_id, state) in dfa.iter() {
        if *same[usize::from(state_id)].get(0).unwrap_or_else(|| &state_id) < state_id {
            continue;
        }

        translate.insert(state_id, curr_state_id.into());
        curr_state_id += 1;

        let mut new_state = dfa::NextElem::new();
        for (symbol, next) in state.iter() {
            let untranslated_actual_next = match same[usize::from(*next)].get(0) {
                Some(x) => min(x, next),
                None => next
            };
            new_state.insert(*symbol, *untranslated_actual_next);
        }
        new_dfa.push(new_state);
    }

    for (_, state) in new_dfa.iter_mut() {
        for (_, next) in state.iter_mut() {
            *next = *translate.get(next).unwrap_or_else(|| next);
        }
    }

    let mut new_finals: HashSet<State> = HashSet::new();
    for f in finals.iter() {
        if let Some(x) = translate.get(f) {
            new_finals.insert(*x);
        }
    }

    *dfa = new_dfa;
    *finals = new_finals;
}
