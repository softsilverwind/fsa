use std::collections::{HashSet, HashMap, VecDeque};
use std::mem;

use crate::{DFA, StateId, SymbolId};

pub type StateString = (Vec<SymbolId>, Vec<StateId>);

#[derive(Debug)]
pub struct DFSInstance<'a>
{
    pub dfa: &'a DFA,
    pub start: StateId,
    pub end: StateId,
    pub discovered: HashSet<StateId>,
    pub finished: HashSet<StateId>,
    pub cycles: HashMap<SymbolId, Vec<StateString>>
}

impl<'a> DFSInstance<'a>
{
    pub fn new(dfa: &'a DFA, start: StateId, end: StateId) -> Self
    {
        DFSInstance {
            dfa, start, end, discovered: HashSet::new(),
            finished: HashSet::new(), cycles: HashMap::new()
        }
    }
}

pub fn dfs(instance: &mut DFSInstance, current: StateId)
{
    instance.discovered.insert(current);

    if current == instance.end {
        instance.cycles.insert(current, vec![(vec![], vec![current])]);

        if current != instance.start {
            return;
        }
    }

    for (&symb, &newstate) in instance.dfa.next[current as usize].iter() {
        if !instance.discovered.contains(&newstate) {
            dfs(instance, newstate);
        }

        if !instance.finished.contains(&newstate) { // Backward edge
            if newstate == instance.end {
                let mut pair = (vec![], vec![newstate]);
                pair.0.push(symb);
                pair.1.push(current);
                instance.cycles.entry(current).or_insert_with(Vec::new).push(pair);
            }
            continue
        }

        let mut cycles: Vec<StateString> = vec![];
        instance.cycles.get(&newstate).unwrap_or(&vec![]).iter().for_each(|cycle| {
            let mut pair = cycle.clone();
            pair.0.push(symb);
            pair.1.push(current);
            cycles.push(pair);
        });

        instance.cycles.entry(current).or_insert_with(Vec::new).append(&mut cycles);
    }

    instance.finished.insert(current);
}

pub fn find_paths(dfa: &DFA, start: StateId, end: StateId) -> Vec<StateString>
{
    let mut instance = DFSInstance::new(dfa, start, end);
    dfs(&mut instance, start);

    instance.cycles.get_mut(&start).map(|cycles| {
        let mut ret = mem::replace(cycles, Vec::new());
        ret.iter_mut().for_each(|cycle| {
            cycle.0.reverse();
            cycle.1.reverse();
        });
        ret
    }).unwrap_or_default()
}


impl super::DFA
{
    pub fn generate<F>(&self, mut f: F)
        where F: FnMut(&[SymbolId]) -> bool
    {
        let mut queue: VecDeque<StateString> = self.finals.iter().flat_map(|i| find_paths(&self, 0, *i)).collect();
        let mut cycles: HashMap<StateId, Vec<StateString>> = HashMap::new();
        let mut visited: HashSet<StateString> = HashSet::new();

        for i in 0..self.next.len() as StateId {
            let mut c = find_paths(&self, i, i);
            c.retain(|elem| !elem.0.is_empty());
            cycles.insert(i, c);
        }

        while let Some(state_string) = queue.pop_front() {
            visited.insert(state_string.clone());
            if !f(&state_string.0) {
                continue;
            }

            for i in 0..state_string.1.len() {
                for cycle in cycles[&state_string.1[i]].iter() {
                    let mut left_states = state_string.1.clone();
                    let mut right_states = left_states.split_off(i+1);
                    left_states.pop();
                    let mut middle_states = cycle.1.clone();

                    let mut new_states = left_states;
                    new_states.append(&mut middle_states);
                    new_states.append(&mut right_states);

                    let mut left_symbols = state_string.0.clone();
                    let mut right_symbols = if i < left_symbols.len() { left_symbols.split_off(i) } else { vec![] };
                    let mut middle_symbols = cycle.0.clone();

                    let mut new_symbols = left_symbols;
                    new_symbols.append(&mut middle_symbols);
                    new_symbols.append(&mut right_symbols);

                    let new_state = (new_symbols, new_states);

                    if !visited.contains(&new_state) {
                        visited.insert(new_state.clone());
                        queue.push_back(new_state);
                    }
                }
            }
        }
    }
}
