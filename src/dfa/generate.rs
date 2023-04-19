use std::collections::{HashMap, HashSet, VecDeque};

use crate::{State, Symbol, DFA};

pub type StateString = (Vec<Symbol>, Vec<State>);

fn dfs(dfa: &DFA, end: State, visited: &mut HashSet<State>, current: State) -> Vec<StateString> {
    if current == end {
        if !visited.is_empty() {
            return vec![(vec![], vec![current])];
        }
    } else {
        visited.insert(current);
    }

    let mut paths = Vec::new();

    for (&symb, &newstate) in dfa.next[current].iter() {
        let paths_rec = if !visited.contains(&newstate) && newstate != current {
            dfs(dfa, end, visited, newstate)
        } else {
            Vec::new()
        };

        for mut path in paths_rec {
            path.0.push(symb);
            path.1.push(current);
            paths.push(path);
        }
    }

    visited.remove(&current);

    paths
}

fn find_paths(dfa: &DFA, start: State, end: State) -> Vec<StateString> {
    let mut paths = dfs(&dfa, end, &mut HashSet::new(), start);

    for path in paths.iter_mut() {
        path.0.reverse();
        path.1.reverse();
    }

    if start == end {
        for (&symb, &newstate) in dfa.next[start].iter() {
            if start == newstate {
                paths.push((vec![symb], vec![start, start]));
            }
        }
    }

    paths
}

impl super::DFA {
    pub fn generate<F>(&self, mut f: F)
    where
        F: FnMut(&[Symbol]) -> bool,
    {
        let mut queue: VecDeque<StateString> = self
            .finals
            .iter()
            .copied()
            .flat_map(|final_state| find_paths(&self, self.initial, final_state))
            .collect();
        let mut cycles: HashMap<State, Vec<StateString>> = HashMap::new();
        let mut visited: HashSet<StateString> = HashSet::new();

        for i in 0..self.next.len() {
            let state = State::from(i);
            let mut c = find_paths(&self, state, state);
            c.retain(|elem| !elem.0.is_empty());
            cycles.insert(state, c);
        }

        if self.finals.contains(&self.initial) {
            if !f(&[]) {
                return;
            }
        }

        while let Some(state_string) = queue.pop_front() {
            visited.insert(state_string.clone());
            if !f(&state_string.0) {
                continue;
            }

            for i in 0..state_string.1.len() {
                for cycle in cycles[&state_string.1[i]].iter() {
                    let mut left_states = state_string.1.clone();
                    let mut right_states = left_states.split_off(i + 1);
                    left_states.pop();
                    let mut middle_states = cycle.1.clone();

                    let mut new_states = left_states;
                    new_states.append(&mut middle_states);
                    new_states.append(&mut right_states);

                    let mut left_symbols = state_string.0.clone();
                    let mut right_symbols = if i < left_symbols.len() {
                        left_symbols.split_off(i)
                    } else {
                        vec![]
                    };
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
