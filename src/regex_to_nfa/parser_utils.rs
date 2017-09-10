use super::super::SymbolId;

#[derive(Debug)]
pub enum Ast
{
    Terminal(SymbolId),
    Cons(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
    Or(Box<Ast>, Box<Ast>)
}