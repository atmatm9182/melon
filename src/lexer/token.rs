#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub lit: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, lit: &'a [u8]) -> Self {
        Self { ty, lit }
    }

    pub fn illegal(lit: &'a [u8]) -> Self {
        Self {
            ty: TokenType::Illegal,
            lit
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Dot,
    Comma,
    Semicolon,
    Colon,

    Plus,
    Minus,
    Star,
    Slash,

    Ident,

    Assign,

    Bang,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,

    Illegal,
}
