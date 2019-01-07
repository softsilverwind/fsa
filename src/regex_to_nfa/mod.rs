use lalrpop_util::lalrpop_mod;

use super::{StateId, NFANext, NFANextElem};

lalrpop_mod!(#[allow(clippy::all)] pub parser, "/src/regex_to_nfa/parser.rs");
mod parser_utils;

use self::parser_utils::{Ast, new_node, new_dummy_node, add_e_transfer, backpatch};

fn regex_to_nfa_inner(ast: &Ast, acc: &mut NFANext) -> (StateId, StateId)
{
    let start = new_dummy_node(acc);
    match ast {
        &Ast::Terminal(id) => {
            new_node(acc, id, vec![start + 2]);
        },
        Ast::Cons(a1, a2) => {
            regex_to_nfa_inner(a1, acc);
            regex_to_nfa_inner(a2, acc);
        },
        Ast::Star(a) => {
            regex_to_nfa_inner(a, acc);
            let end = new_dummy_node(acc);
            add_e_transfer(acc, end, start); // might return
            add_e_transfer(acc, start, end); // might skip
        }
        Ast::Or(a1, a2) => {
            let (_, a1_end) = regex_to_nfa_inner(a1, acc);
            let (a2_start, _) = regex_to_nfa_inner(a2, acc);
            let end = new_dummy_node(acc);
            backpatch(acc, a1_end, a2_start, end); // a1 should continue after a2_end
            add_e_transfer(acc, start, a2_start);  // Make start skip to a2
        },
        Ast::Optional(a) => {
            regex_to_nfa_inner(a, acc);
            let end = new_dummy_node(acc);
            add_e_transfer(acc, start, end); // might skip
        },
        &Ast::Range(ref a, min, max) => {
            for _ in 0..min {
                regex_to_nfa_inner(a, acc);
                new_dummy_node(acc);
            }
            for _ in min..max {
                let n1 = new_dummy_node(acc);
                regex_to_nfa_inner(a, acc);
                let n2 = new_dummy_node(acc);
                add_e_transfer(acc, n1, n2);
            }
        }
    }
    (start, acc.len() as StateId - 1)
}

pub fn regex_to_nfa(regex: &str) -> Result<NFANext, String>
{
    let ast = match parser::RegexParser::new().parse(regex) {
        Ok(x) => x,
        Err(x) => return Err(format!("{:?}", x))
    };
    let mut ret: NFANext = NFANext::new();
    regex_to_nfa_inner(&ast, &mut ret);
    ret.push(NFANextElem::new());
    Ok(ret)
}
