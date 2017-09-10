use super::{StateId, NFANext, NFANextElem};

mod parser;
mod parser_utils;

use self::parser_utils::Ast;

fn regex_to_nfa_inner(ast: Box<Ast>, acc: &mut NFANext)
{
    let ast_deref = *ast;
    match ast_deref {
        Ast::Terminal(id) => {
            let mut hmap: NFANextElem = NFANextElem::new();
            let end = acc.len() as StateId;
            hmap.insert(id, vec![end + 1]);
            acc.push(hmap);
        },
        Ast::Cons(a1, a2) => {
            regex_to_nfa_inner(a1, acc);
            regex_to_nfa_inner(a2, acc);
        },
        Ast::Star(a) => {
            let mut hmap: NFANextElem = NFANextElem::new();
            let start = acc.len() as StateId;
            hmap.insert(-1, vec![start + 1]);
            acc.push(hmap);
            let mut hmap: NFANextElem = NFANextElem::new();
            regex_to_nfa_inner(a, acc);
            let end = acc.len() as StateId;
            hmap.insert(-1, vec![start, end + 1]); // might return or continue
            acc[start as usize].entry(-1).or_insert(Vec::new()).push(end + 1); // might skip
            acc.push(hmap);

            // New node to ensure all returns are from end, for backpatch
            let mut hmap: NFANextElem = NFANextElem::new();
            let new_node = acc.len() as StateId;
            hmap.insert(-1, vec![new_node + 1]);
            acc.push(hmap);
        }
        Ast::Or(a1, a2) => {
            let mut hmap: NFANextElem = NFANextElem::new();
            let new_node = acc.len() as StateId;
            hmap.insert(-1, vec![new_node + 1]);
            acc.push(hmap);
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
            acc[new_node as usize].entry(-1).or_insert(Vec::new()).push(a2_start);
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
    regex_to_nfa_inner(ast, &mut ret);
    ret.push(NFANextElem::new());
    Ok(ret)
}