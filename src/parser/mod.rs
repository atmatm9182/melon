use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    cur: Option<Token<'a>>,
    peek: Option<Token<'a>>,
    lex: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lex: Lexer<'a>) -> Self {
        let cur = lex.next();
        let peek = lex.next();
        Self { cur, peek, lex }
    }
}
