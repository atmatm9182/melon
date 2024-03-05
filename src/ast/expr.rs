use std::str::FromStr;

use super::Ident;

#[derive(Debug)]
pub enum Expr<'a> {
    IntLit(i128),
    Binary(BinaryExpr<'a>),
    Ident(Ident<'a>)
}

pub type IntLitParseError = <i128 as FromStr>::Err;

impl<'a> Expr<'a> {
    pub fn parse_int(lit: &'_ [u8]) -> Result<Self, IntLitParseError> {
        let as_str = std::str::from_utf8(lit).unwrap();
        let i: i128 = as_str.parse()?;
        Ok(Self::IntLit(i))
    }
}

#[derive(Debug)]
pub struct BinaryExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
    pub op: BinOp,
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Mul,
    Div,
}
