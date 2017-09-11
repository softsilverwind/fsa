use super::{StateId, SymbolId, NFANext, NFANextElem};

mod parser;
mod parser_utils;

use self::parser_utils::Ast;

fn new_node (acc: &mut NFANext, id: SymbolId, next: Vec<StateId>)
{
    let mut hmap: NFANextElem = NFANextElem::new();
    hmap.insert(id, next);
    acc.push(hmap);
}

fn new_dummy_node(acc: &mut NFANext) -> StateId
{
    let len = acc.len() as StateId;
    new_node(acc, -1, vec![len + 1]);
    len
}

fn regex_to_nfa_inner(ast: &Ast, acc: &mut NFANext)
{
    match ast {
        &Ast::Terminal(id) => {
            let end = acc.len() as StateId;
            new_node(acc, id, vec![end + 1]);
        },
        &Ast::Cons(ref a1, ref a2) => {
            regex_to_nfa_inner(a1, acc);
            regex_to_nfa_inner(a2, acc);
        },
        &Ast::Star(ref a) => {
            let start = new_dummy_node(acc);
            regex_to_nfa_inner(a, acc);
            let end = acc.len() as StateId;
            new_node(acc, -1, vec![start, end + 1]); // might return or continue
            acc[start as usize].get_mut(&-1).unwrap().push(end + 1); // might skip
            new_dummy_node(acc);
        }
        &Ast::Or(ref a1, ref a2) => {
            let dummy_node = new_dummy_node(acc);
            regex_to_nfa_inner(a1, acc);
            let a1_end = acc.len() as StateId - 1;
            let a2_start = acc.len() as StateId;
            regex_to_nfa_inner(a2, acc);
            let a2_end = acc.len() as StateId - 1;
            // Backpatch a1 to a2_end
            for (_, val) in acc[a1_end as usize].iter_mut() {
                for x in val.iter_mut() {
                    if *x == a2_start {
                        *x = a2_end + 1;
                    }
                }
            }
            // Make new_node skip to a2
            acc[dummy_node as usize].get_mut(&-1).unwrap().push(a2_start);
        },
        &Ast::Optional(ref a) => {
            let start = new_dummy_node(acc);
            regex_to_nfa_inner(a, acc);
            let end = acc.len() as StateId;
            acc[start as usize].get_mut(&-1).unwrap().push(end + 1); // might skip
            new_dummy_node(acc);
        },
        &Ast::Range(ref a, min, max) => {
            new_dummy_node(acc);
            for _ in 0..min {
                regex_to_nfa_inner(a, acc);
            }
            for _ in min..max {
                let start = new_dummy_node(acc);
                regex_to_nfa_inner(a, acc);
                let end = acc.len() as StateId;
                acc[start as usize].get_mut(&-1).unwrap().push(end); // might skip
            }
            new_dummy_node(acc);
        }
    }
}

pub fn regex_to_nfa(regex: &str) -> Result<NFANext, String>
{
    let ast = match parser::parse_Regex(regex) {
        Ok(x) => x,
        Err(x) => return Err(format!("{:?}", x))
    };
    let mut ret: NFANext = NFANext::new();
    regex_to_nfa_inner(&ast, &mut ret);
    ret.push(NFANextElem::new());
    Ok(ret)
}