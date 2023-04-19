use std::{error::Error, iter};

use lalrpop_util::lalrpop_mod;

use crate::{nfa, State};

lalrpop_mod!(#[allow(clippy::all)] pub parser, "/src/nfa/regex_parser/parser.rs");
mod parser_utils;

use self::parser_utils::{add_e_transfer, backpatch, new_dummy_node, new_node, Ast};

fn parse_rec(ast: &Ast, acc: &mut nfa::NextElems) -> (State, State) {
    let start = new_dummy_node(acc);
    match ast {
        &Ast::Terminal(id) => {
            new_node(acc, id, iter::once(State(start.0 + 2)).collect());
        }
        Ast::Cons(a1, a2) => {
            parse_rec(a1, acc);
            parse_rec(a2, acc);
        }
        Ast::Star(a) => {
            parse_rec(a, acc);
            let end = new_dummy_node(acc);
            add_e_transfer(acc, end, start); // might return
            add_e_transfer(acc, start, end); // might skip
        }
        Ast::Or(a1, a2) => {
            let (_, a1_end) = parse_rec(a1, acc);
            let (a2_start, _) = parse_rec(a2, acc);
            let end = new_dummy_node(acc);
            backpatch(acc, a1_end, a2_start, end); // a1 should continue after a2_end
            add_e_transfer(acc, start, a2_start); // Make start skip to a2
        }
        Ast::Optional(a) => {
            parse_rec(a, acc);
            let end = new_dummy_node(acc);
            add_e_transfer(acc, start, end); // might skip
        }
        &Ast::Range(ref a, min, max) => {
            for _ in 0..min {
                parse_rec(a, acc);
                new_dummy_node(acc);
            }
            for _ in min..max {
                let n1 = new_dummy_node(acc);
                parse_rec(a, acc);
                let n2 = new_dummy_node(acc);
                add_e_transfer(acc, n1, n2);
            }
        }
    }
    (start, (acc.len() - 1).into())
}

pub fn parse(regex: &str) -> Result<nfa::NextElems, Box<dyn Error>> {
    let ast = parser::RegexParser::new()
        .parse(regex)
        .map_err(|x| x.to_string())?;
    let mut ret: nfa::NextElems = nfa::NextElems::new();
    parse_rec(&ast, &mut ret);
    ret.push(nfa::NextElem::new());
    Ok(ret)
}
