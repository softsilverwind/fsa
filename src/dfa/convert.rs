use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use nicole::{typedvec::TypedVec, IdLike};

use crate::{dfa, nfa, State, Symbol, DFA, NFA};

pub fn e_closure(nfa: &nfa::NextElems) -> TypedVec<State, BTreeSet<State>> {
    let mut ret: TypedVec<State, BTreeSet<State>> = TypedVec::new();
    *ret = vec![BTreeSet::new(); nfa.len()];
    let mut stack: Vec<State> = Vec::new();

    for initial in 0..nfa.len() {
        let initial_state = State::from(initial);
        let mut visited: TypedVec<State, bool> = TypedVec::new();
        *visited = vec![false; nfa.len()];

        stack.push(initial_state);

        while let Some(x) = stack.pop() {
            if !visited[x] {
                ret[initial_state].insert(x.into());
                if let Some(neighbors) = nfa[x].get(&Symbol::null()) {
                    stack.extend(neighbors);
                }
            }
            visited[x] = true;
        }
    }

    ret
}

pub fn nfa_to_dfa(nfa: NFA) -> DFA {
    let mut queue: VecDeque<BTreeSet<State>> = VecDeque::new();
    let mut translate: HashMap<BTreeSet<State>, State> = HashMap::new();
    let mut ret = dfa::NextElems::new();
    let mut finals: HashSet<State> = HashSet::new();
    let ecl = e_closure(&nfa.next);
    let initial: BTreeSet<State> = nfa
        .initials
        .iter()
        .copied()
        .flat_map(|x| ecl[x].clone())
        .collect();

    queue.push_back(initial.clone());
    translate.insert(initial, 0.into());
    let mut max_state = 1;

    while let Some(vec) = queue.pop_front() {
        let mut ret2 = dfa::NextElem::new();

        if vec.iter().any(|x| nfa.finals.contains(&x)) {
            finals.insert(translate[&vec]);
        }

        let mut next_states = nfa::NextElem::new();
        for state in vec {
            for (symbol, next) in nfa.next[state].iter() {
                if symbol.is_null() {
                    continue;
                }

                let entry = next_states.entry(*symbol).or_default();
                entry.extend(next.iter().copied().flat_map(|n| ecl[n].iter()));
            }
        }

        for (symbol, next) in next_states.into_iter() {
            let next_state_id = translate.get(&next).copied().unwrap_or_else(|| {
                queue.push_back(next.clone());
                translate.insert(next.clone(), max_state.into());
                max_state += 1;
                (max_state - 1).into()
            });

            ret2.insert(symbol, next_state_id);
        }

        ret.push(ret2);
    }

    DFA {
        next: ret,
        initial: State(0),
        finals,
    }
}
