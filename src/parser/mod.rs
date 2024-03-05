use std::{cell::RefCell, iter::Peekable};

use crate::{
    ast::{Expr, Ident, IntLitParseError, LetStatement, Program, Statement},
    lexer::{Lexer, Token, TokenType},
};

pub struct Parser<'a, I: Iterator<Item = Token<'a>>> {
    tokens: RefCell<Peekable<I>>,
}

#[derive(Debug)]
pub enum Error<'t> {
    None,
    Eof,
    WrongToken {
        expected: Vec<TokenType>,
        got: Token<'t>,
    },
    BadIntLit(IntLitParseError),
}

impl<'t> Error<'t> {
    pub fn wrong_token(expected: Vec<TokenType>, got: Token<'t>) -> Self {
        Self::WrongToken { expected, got }
    }
}

pub type ParserResult<'a, T> = Result<T, Error<'a>>;

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token<'a>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            tokens: iter.peekable().into(),
        }
    }

    pub fn parse_program(&self) -> ParserResult<Program> {
        let mut stmts = vec![];
        loop {
            let res = self.parse_stmt();
            match res {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    if matches!(err, | Error::None) {
                        break;
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        Ok(Program(stmts))
    }

    #[inline]
    fn advance(&self) {
        self.tokens.borrow_mut().next();
    }

    #[inline]
    fn cur(&self) -> ParserResult<Token<'a>> {
        let mut tokens = self.tokens.borrow_mut();
        let cur = tokens.peek();
        cur.cloned().ok_or(Error::Eof)
    }

    fn expect_cur(&self, tt: TokenType) -> ParserResult<Token<'a>> {
        let cur = self.cur()?;
        if tt == cur.ty {
            self.advance();
            Ok(cur)
        } else {
            Err(Error::wrong_token(vec![tt], cur))
        }
    }

    fn parse_stmt(&self) -> ParserResult<Statement> {
        let tok = self.cur();

        match tok {
            Ok(tok) => match tok.ty {
                TokenType::Let => self.parse_let(),
                _ => todo!(),
            },
            Err(Error::Eof) => Err(Error::None),
            Err(e) => Err(e),
        }
    }

    fn parse_let(&self) -> ParserResult<Statement> {
        self.advance();
        let var = self.expect_cur(TokenType::Ident)?.lit;
        self.expect_cur(TokenType::Assign)?;
        let expr = self.parse_expr()?;
        self.expect_cur(TokenType::Semicolon)?;

        Ok(Statement::Let(LetStatement { var, expr }))
    }

    fn parse_int(&self, lit: &'_ [u8]) -> ParserResult<Expr<'a>> {
        let e = Expr::parse_int(lit).map_err(Error::BadIntLit)?;
        self.advance();
        return Ok(e);
    }

    fn parse_expr(&self) -> ParserResult<Expr<'a>> {
        let tok = self.cur()?;
        match tok.ty {
            TokenType::Ident => Ok(Expr::Ident(tok.lit)),
            TokenType::IntLit => self.parse_int(tok.lit),
            _ => Err(Error::wrong_token(
                vec![TokenType::Ident, TokenType::IntLit],
                tok,
            )),
        }
    }
}
