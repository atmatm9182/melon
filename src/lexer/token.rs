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
            lit,
        }
    }

    /// If `lit` is a keyword, returns a new token of a corresponding type, otherwise a token
    /// of `TokenType::Ident` type
    pub fn keyword(lit: &'a [u8]) -> Self {
        let ty = match lit {
            b"let" => TokenType::Let,
            b"fn" => TokenType::Fn,
            _ => TokenType::Ident,
        };

        Self { ty, lit }
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

    Let,
    Fn,

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
