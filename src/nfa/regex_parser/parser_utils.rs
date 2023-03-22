use crate::{
    State, Symbol, nfa
};

use nicole::IdLike;

#[derive(Debug)]
pub enum Ast
{
    Terminal(Symbol),
    Cons(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
    Or(Box<Ast>, Box<Ast>),
    Optional(Box<Ast>),
    Range(Box<Ast>, i32, i32)
}

pub fn new_node(acc: &mut nfa::NextElems, symbol: Symbol, next: Vec<State>) -> State
{
    let len: State = acc.len().into();
    let mut hmap = nfa::NextElem::new();
    hmap.insert(symbol, next);
    acc.push(hmap);
    len
}

pub fn new_dummy_node(acc: &mut nfa::NextElems) -> State
{
    new_node(acc, Symbol::null(), vec![(acc.len() + 1).into()])
}

pub fn add_e_transfer(acc: &mut nfa::NextElems, from: State, to: State)
{
    acc[from].get_mut(&Symbol::null()).unwrap().push(to);
}

pub fn backpatch(acc: &mut nfa::NextElems, state: State, from: State, to: State)
{
    for (_, val) in acc[state].iter_mut() {
        for x in val.iter_mut() {
            if *x == from {
                *x = to;
            }
        }
    }
}
