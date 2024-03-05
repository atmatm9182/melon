use super::{expr::Expr, Ident};

#[derive(Debug)]
pub struct LetStatement<'a> {
    pub var: Ident<'a>,
    pub expr: Expr<'a>,
}

#[derive(Debug)]
pub enum Statement<'a> {
    Let(LetStatement<'a>)
}
