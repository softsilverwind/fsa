use std::collections::HashSet;

use proptest::prelude::*;
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};

use crate::{Symbol, DFA, NFA};

#[derive(Clone, Debug)]
pub enum Ast {
    Terminal(String),
    Cons(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
    Or(Box<Ast>, Box<Ast>),
    Optional(Box<Ast>),
    Range(Box<Ast>, i32, i32),
}

impl Ast {
    pub fn to_string(&self) -> String {
        fn to_string_rec(ast: &Ast, acc: &mut String) {
            match ast {
                Ast::Terminal(x) => acc.push_str(x),
                Ast::Cons(x, y) => {
                    to_string_rec(x, acc);
                    to_string_rec(y, acc);
                }
                Ast::Star(x) => {
                    to_string_rec(x, acc);
                    acc.push('*');
                }
                Ast::Or(x, y) => {
                    to_string_rec(x, acc);
                    acc.push('|');
                    to_string_rec(y, acc);
                }
                Ast::Optional(x) => {
                    // Workaround to avoid regex engines treating "a{1,1}?" as a non-greedy match
                    acc.push('(');
                    to_string_rec(x, acc);
                    acc.push(')');
                    acc.push('?');
                }
                Ast::Range(x, r1, r2) => {
                    to_string_rec(x, acc);
                    acc.push_str(&format!("{{{},{}}}", r1, r2));
                }
            }
        }

        let mut ret = String::new();
        to_string_rec(self, &mut ret);
        ret
    }
}

fn arb_ast() -> impl Strategy<Value = Ast> {
    let leaf = prop_oneof!["[a-z]".prop_map(Ast::Terminal)].boxed();
    leaf.prop_recursive(3, 4, 4, |inner| {
        let flat_map = (1..10).prop_flat_map(|a| (Just(a), a..10));
        prop_oneof![
            (inner.clone(), inner.clone()).prop_map(|(x, y)| Ast::Cons(Box::new(x), Box::new(y))),
            inner.clone().prop_map(|x| Ast::Star(Box::new(x))),
            (inner.clone(), inner.clone()).prop_map(|(x, y)| Ast::Or(Box::new(x), Box::new(y))),
            inner.clone().prop_map(|x| Ast::Optional(Box::new(x))),
            (inner, flat_map).prop_map(|(x, (r1, r2))| Ast::Range(Box::new(x), r1, r2)),
        ]
    })
}

// TODO: These tests are not exhaustive since they rely on the regex created NFAs/DFAs.
proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        s.parse::<NFA>().ok();
    }

    #[test]
    fn dfa_accepts(ast in arb_ast()) {
        let regexp = ast.to_string();
        let dfa: DFA = regexp.parse::<NFA>().map_err(|_| TestCaseError::fail("Failed to parse NFA".to_string()))?.into();

        let mut gen = Generator::new(&regexp, rand::thread_rng(), DEFAULT_MAX_REPEAT)?;

        let inputs: HashSet<String> = (0..100).map(|_| {
            let mut buffer = vec![];
            gen.generate(&mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        })
        .collect();

        for input in inputs {
            println!("{regexp} {input} ::> {dfa:?}");
            prop_assert!(dfa.matches(&input.bytes().map(|b| Symbol::from_u8(b)).collect::<Vec<_>>()));
        }
    }

    #[test]
    fn minimized_dfa_accepts(ast in arb_ast()) {
        let regexp = ast.to_string();
        let dfa = DFA::from(regexp.parse::<NFA>().map_err(|_| TestCaseError::fail("Failed to parse NFA".to_string()))?).minimize();

        let mut gen = Generator::new(&regexp, rand::thread_rng(), DEFAULT_MAX_REPEAT)?;

        let inputs: HashSet<String> = (0..100).map(|_| {
            let mut buffer = vec![];
            gen.generate(&mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        })
        .collect();

        for input in inputs {
            println!("{regexp} {input} ::> {dfa:?}");
            prop_assert!(dfa.matches(&input.bytes().map(|b| Symbol::from_u8(b)).collect::<Vec<_>>()));
        }
    }

    #[test]
    fn rev_dfa_accepts_rev_str(ast in arb_ast()) {
        let regexp = ast.to_string();
        let dfa: DFA = regexp.parse::<NFA>().map_err(|_| TestCaseError::fail("Failed to parse NFA".to_string()))?.into();

        let mut gen = Generator::new(&regexp, rand::thread_rng(), DEFAULT_MAX_REPEAT)?;
        let inputs: HashSet<String> = (0..100).map(|_| {
            let mut buffer = vec![];
            gen.generate(&mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        })
        .collect();

        let rev = DFA::from(dfa.reverse()).minimize();

        for input in inputs {
            let rev_input = input.bytes().rev().map(|b| Symbol::from_u8(b)).collect::<Vec<_>>();
            println!("{regexp} reversed {rev_input:?} ::> {rev:?}");
            prop_assert!(rev.matches(&rev_input));
        }
    }

    #[test]
    fn generated_match_regex(ast in arb_ast()) {
        let regexp = ast.to_string();
        let dfa = DFA::from(regexp.parse::<NFA>().map_err(|_| TestCaseError::fail("Failed to parse NFA".to_string()))?).minimize();
        let re = regex::Regex::new(&regexp)?;
        let mut errors = Vec::new();

        let mut count = 100;

        dfa.generate(|symbols| {
            let s: String = symbols.iter().copied().map(|x| (usize::from(x) as u8 + 97) as char).collect();
            count -= 1;
            if !re.is_match(&s) {
                errors.push(s);
            }
            count > 0
        });

        println!("{regexp} {errors:?}");
        prop_assert!(errors.is_empty());
    }

    #[test]
    fn gen_stuff(ast in arb_ast()) {
        let regexp = ast.to_string();
        let dfa = DFA::from(regexp.parse::<NFA>().map_err(|_| TestCaseError::fail("Failed to parse NFA".to_string()))?).minimize();
        let max_count = 100;

        let mut gen = Generator::new(&regexp, rand::thread_rng(), DEFAULT_MAX_REPEAT)?;
        let inputs: HashSet<String> = (0..max_count).map(|_| {
            let mut buffer = vec![];
            gen.generate(&mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        })
        .collect();

        let mut count = max_count;

        let mut generated = Vec::new();
        dfa.generate(|symbols| {
            let s: String = symbols.iter().copied().map(|x| (usize::from(x) as u8 + 97) as char).collect();
            generated.push(s);

            count -= 1;
            count > 0
        });

        println!("{generated:?} {inputs:?}");
        prop_assert!(generated.len() >= inputs.len());
    }
}
