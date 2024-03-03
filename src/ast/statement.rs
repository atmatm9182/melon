use super::{expr::Expr, Ident};

pub struct LetStatement<'a> {
    pub var: Ident<'a>,
    pub expr: Expr,
}

pub enum Statement<'a> {
    Let(LetStatement<'a>)
}
