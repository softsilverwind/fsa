use std::collections::{HashMap, HashSet, VecDeque};

use super::{StateId, NFANext, DFANext, NFANextElem, DFANextElem};

pub fn e_closure(nfa: &NFANext) -> Vec<Vec<StateId>>
{
    let mut ret: Vec<Vec<StateId>> = vec![Vec::new(); nfa.len()];
    let mut stack: Vec<StateId> = Vec::new();

    for initial in 0..(nfa.len() as StateId - 1) {
        let mut visited: Vec<bool> = vec![false; nfa.len()];
        stack.push(initial);

        while let Some(x) = stack.pop() {
            if !visited[x as usize] {
                ret[initial as usize].push(x);
                if let Some(neighbors) = nfa[x as usize].get(&-1) {
                    stack.extend(neighbors);
                }
            }
            visited[x as usize] = true;
        }
    }

    for x in ret.iter_mut() {
        x.sort();
        x.dedup();
    }

    ret
}

pub fn nfa_to_dfa(nfa: &NFANext) -> (DFANext,HashSet<StateId>)
{
    let mut queue: VecDeque<Vec<StateId>> = VecDeque::new();
    let mut translate: HashMap<Vec<StateId>, StateId> = HashMap::new();
    let mut ret: DFANext = DFANext::new();
    let mut finals: HashSet<StateId> = HashSet::new();
    let ecl = e_closure(&nfa);
    let initial: Vec<StateId> = ecl[0].clone();

    queue.push_back(initial.clone());
    translate.insert(initial, 0);
    let mut max_state = 1;

    while let Some(vec) = queue.pop_front() {
        let mut ret2: DFANextElem = DFANextElem::new();

        for x in vec.iter() {
            if *x == nfa.len() as StateId - 1 {
                finals.insert(translate[&vec]);
            }
        }

        let mut next_states: NFANextElem = NFANextElem::new();
        for elem in vec {
            for (symbol, next) in nfa[elem as usize].iter() {
                if *symbol == -1 {
                    continue
                }

                let entry = next_states.entry(*symbol).or_insert(Vec::new());
                entry.extend(next);
                for n in next {
                    entry.extend(ecl[*n as usize].iter());
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
                translate.insert(next.clone(), max_state);
                max_state += 1;
                max_state - 1
            };

            ret2.insert(symbol, next_state_id);
        }

        ret.push(ret2);
    }

    (ret, finals)
}