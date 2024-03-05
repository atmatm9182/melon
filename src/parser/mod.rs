use std::{cell::RefCell, iter::Peekable};

use crate::{
    ast::{
        BlockStatement, Expr, FunctionDef, Ident, IntLitParseError, LetStatement, ParamList,
        Program, Statement,
    },
    lexer::{Token, TokenType},
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

    fn expect_cur_one_of(&self, ts: Vec<TokenType>) -> ParserResult<Token<'a>> {
        let cur = self.cur()?;
        if ts.contains(&cur.ty) {
            self.advance();
            Ok(cur)
        } else {
            Err(Error::wrong_token(ts, cur))
        }
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

    fn parse_stmt(&self) -> ParserResult<Statement<'a>> {
        let tok = self.cur();

        match tok {
            Ok(tok) => match tok.ty {
                TokenType::Let => self.parse_let(),
                TokenType::Fn => self.parse_fn(),
                _ => Err(Error::wrong_token(vec![TokenType::Let, TokenType::Fn], tok))
            },
            Err(Error::Eof) => Err(Error::None),
            Err(e) => Err(e),
        }
    }

    fn parse_let(&self) -> ParserResult<Statement<'a>> {
        self.advance();
        let var = self.parse_ident()?;
        self.expect_cur(TokenType::Assign)?;
        let expr = self.parse_expr()?;
        self.expect_cur(TokenType::Semicolon)?;

        Ok(Statement::Let(LetStatement { var, expr }))
    }

    fn parse_int(&self) -> ParserResult<Expr<'a>> {
        let cur = self.expect_cur(TokenType::IntLit)?;
        let i = Expr::parse_int(cur.lit).map_err(Error::BadIntLit)?;
        Ok(i)
    }

    fn parse_ident(&self) -> ParserResult<Ident<'a>> {
        let cur = self.expect_cur(TokenType::Ident)?;
        let ident = std::str::from_utf8(cur.lit).unwrap();
        Ok(ident)
    }

    fn parse_expr(&self) -> ParserResult<Expr<'a>> {
        let tok = self.cur()?;
        match tok.ty {
            TokenType::Ident => Ok(Expr::Ident(self.parse_ident()?)),
            TokenType::IntLit => self.parse_int(),
            _ => Err(Error::wrong_token(
                vec![TokenType::Ident, TokenType::IntLit],
                tok,
            )),
        }
    }

    fn parse_fn(&self) -> ParserResult<Statement<'a>> {
        self.advance();
        let name = self.parse_ident()?;
        let params = self.parse_params()?;
        self.expect_cur(TokenType::Colon)?;
        let return_type = self.parse_ident()?;

        let body = self.parse_block()?;

        Ok(Statement::FunctionDef(FunctionDef { name, params, body, return_type }))
    }

    fn parse_block(&self) -> ParserResult<BlockStatement<'a>> {
        self.advance();

        let mut stmts = vec![];
        let mut cur_token = self.cur()?;

        while cur_token.ty != TokenType::CloseBrace {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);

            cur_token = self.cur()?;
        }

        self.advance();

        Ok(BlockStatement(stmts))
    }

    fn parse_params(&self) -> ParserResult<ParamList<'a>> {
        self.advance();

        let mut params = vec![];

        loop {
            let param_name = self.parse_ident()?;
            self.expect_cur(TokenType::Colon)?;
            let param_type = self.parse_ident()?;
            params.push((param_name, param_type));

            let cur_token = self.cur()?;
            match cur_token.ty {
                TokenType::Comma => self.advance(),
                TokenType::CloseParen => break,
                _ => return Err(Error::wrong_token(vec![TokenType::Comma, TokenType::CloseParen], cur_token))
            }
        }

        self.advance();

        Ok(ParamList(params))
    }
}
