use std::collections::{HashMap, HashSet, VecDeque};

use nicole::IdLike;

use crate::{State, nfa, dfa, Symbol};

pub fn e_closure(nfa: &nfa::NextElemsView) -> Vec<Vec<State>>
{
    let mut ret: Vec<Vec<State>> = vec![Vec::new(); nfa.len()];
    let mut stack: Vec<State> = Vec::new();

    for initial in 0..nfa.len() - 1 {
        let mut visited: Vec<bool> = vec![false; nfa.len()];
        stack.push(initial.into());

        while let Some(x) = stack.pop() {
            if !visited[usize::from(x)] {
                ret[initial].push(x.into());
                if let Some(neighbors) = nfa[usize::from(x)].get(&Symbol::null()) {
                    stack.extend(neighbors);
                }
            }
            visited[usize::from(x)] = true;
        }
    }

    for x in ret.iter_mut() {
        x.sort();
        x.dedup();
    }

    ret
}

pub fn nfa_to_dfa(nfa: &nfa::NextElemsView) -> (dfa::NextElems, HashSet<State>)
{
    let mut queue: VecDeque<Vec<State>> = VecDeque::new();
    let mut translate: HashMap<Vec<State>, State> = HashMap::new();
    let mut ret = dfa::NextElems::new();
    let mut finals: HashSet<State> = HashSet::new();
    let ecl = e_closure(&nfa);
    let initial: Vec<State> = ecl[0].clone();

    queue.push_back(initial.clone());
    translate.insert(initial, 0.into());
    let mut max_state = 1;

    while let Some(vec) = queue.pop_front() {
        let mut ret2 = dfa::NextElem::new();

        for x in vec.iter() {
            if usize::from(*x) == nfa.len() - 1 {
                finals.insert(translate[&vec]);
            }
        }

        let mut next_states = nfa::NextElem::new();
        for elem in vec {
            for (symbol, next) in nfa[usize::from(elem)].iter() {
                if symbol.is_null() {
                    continue
                }

                let entry = next_states.entry(*symbol).or_insert_with(Vec::new);
                entry.extend(next);
                for n in next {
                    entry.extend(ecl[usize::from(*n)].iter());
                }

                entry.sort();
                entry.dedup();
            }
        }

        for (symbol, next) in next_states {
            let next_state_id = if translate.contains_key(&next) {
                translate[&next]
            }
            else {
                queue.push_back(next.clone());
                translate.insert(next.clone(), max_state.into());
                max_state += 1;
                (max_state - 1).into()
            };

            ret2.insert(symbol, next_state_id);
        }

        ret.push(ret2);
    }

    (ret, finals)
}
