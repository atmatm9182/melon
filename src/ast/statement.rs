use super::{expr::Expr, Ident};

#[derive(Debug)]
pub struct LetStatement<'a> {
    pub var: Ident<'a>,
    pub expr: Expr<'a>,
}

#[derive(Debug)]
pub struct BlockStatement<'a>(pub Vec<Statement<'a>>);

#[derive(Debug)]
pub struct ParamList<'a>(pub Vec<(Ident<'a>, Ident<'a>)>);

#[derive(Debug)]
pub struct FunctionDef<'a> {
    pub name: Ident<'a>,
    pub params: ParamList<'a>,
    pub body: BlockStatement<'a>,
    pub return_type: Ident<'a>,
}

#[derive(Debug)]
pub enum Statement<'a> {
    Let(LetStatement<'a>),
    Block(BlockStatement<'a>),
    FunctionDef(FunctionDef<'a>),
}
