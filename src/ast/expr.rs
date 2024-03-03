pub enum Expr {
    IntLit(i128),
    Binary(BinaryExpr),
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinOp,
}

pub enum BinOp {
    Plus,
    Minus,
    Mul,
    Div,
}
