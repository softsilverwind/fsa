use super::super::{StateId, SymbolId, NFANext, NFANextElem};

#[derive(Debug)]
pub enum Ast
{
    Terminal(SymbolId),
    Cons(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
    Or(Box<Ast>, Box<Ast>),
    Optional(Box<Ast>),
    Range(Box<Ast>, i32, i32)
}

pub fn new_node (acc: &mut NFANext, symbol: SymbolId, next: Vec<StateId>) -> StateId
{
    let len = acc.len() as StateId;
    let mut hmap: NFANextElem = NFANextElem::new();
    hmap.insert(symbol, next);
    acc.push(hmap);
    len
}

pub fn new_dummy_node(acc: &mut NFANext) -> StateId
{
    let len = acc.len() as StateId;
    new_node(acc, -1, vec![len + 1])
}

pub fn add_e_transfer(acc: &mut NFANext, from: StateId, to: StateId)
{
    acc[from as usize].get_mut(&-1).unwrap().push(to);
}

pub fn backpatch(acc: &mut NFANext, state: StateId, from: StateId, to: StateId)
{
    for (_, val) in acc[state as usize].iter_mut() {
        for x in val.iter_mut() {
            if *x == from {
                *x = to;
            }
        }
    }
}