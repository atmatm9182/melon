use std::ops::Range;

mod token;

pub use self::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    contents: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    #[inline(always)]
    fn advance(&mut self) {
        self.pos += 1
    }

    #[inline(always)]
    fn retreat(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    #[inline(always)]
    fn is_at_end(&self) -> bool {
        self.pos >= self.contents.len()
    }

    #[inline(always)]
    fn cur_char(&self) -> u8 {
        self.contents[self.pos]
    }

    #[inline(always)]
    fn peek_char(&self) -> Option<u8> {
        self.contents.get(self.pos + 1).copied()
    }

    #[inline(always)]
    fn slice_len(&self, len: usize) -> &'a [u8] {
        self.slice(self.pos..self.pos + len)
    }

    #[inline(always)]
    fn slice(&self, range: Range<usize>) -> &'a [u8] {
        &self.contents[range]
    }

    fn read_while(&mut self, pred: fn(u8) -> bool) -> &'a [u8] {
        let start = self.pos;
        while !self.is_at_end() {
            let c = self.cur_char();
            if !pred(c) {
                break;
            }
            self.advance();
        }

        self.slice(start..self.pos)
    }

    #[inline(always)]
    fn read_ident(&mut self) -> &'a [u8] {
        self.read_while(pred::is_valid_ident)
    }

    #[inline(always)]
    fn read_int(&mut self) -> &'a [u8] {
        self.read_while(|c| c.is_ascii_digit())
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.cur_char().is_ascii_whitespace() {
            self.advance()
        }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            contents: value.as_bytes(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if self.is_at_end() {
            return None;
        }

        let token = match self.cur_char() {
            b'.' => Token::new(TokenType::Dot, self.slice_len(1)),
            b',' => Token::new(TokenType::Comma, self.slice_len(1)),
            b':' => Token::new(TokenType::Colon, self.slice_len(1)),
            b';' => Token::new(TokenType::Semicolon, self.slice_len(1)),
            b'+' => Token::new(TokenType::Plus, self.slice_len(1)),
            b'-' => Token::new(TokenType::Minus, self.slice_len(1)),
            b'*' => Token::new(TokenType::Star, self.slice_len(1)),
            b'/' => Token::new(TokenType::Slash, self.slice_len(1)),
            b'=' => {
                let peek = self.peek_char();
                if peek.is_some_and(|c| c == b'=') {
                    self.advance();
                    Token::new(TokenType::Eq, self.slice(self.pos - 1..self.pos + 1))
                } else {
                    Token::new(TokenType::Assign, self.slice_len(1))
                }
            }
            b'!' => {
                let peek = self.peek_char();
                if peek.is_some_and(|c| c == b'=') {
                    self.advance();
                    Token::new(TokenType::Neq, self.slice(self.pos - 1..self.pos + 1))
                } else {
                    Token::new(TokenType::Bang, self.slice_len(1))
                }
            }
            b'>' => {
                let peek = self.peek_char();
                if peek.is_some_and(|c| c == b'=') {
                    self.advance();
                    Token::new(TokenType::Gte, self.slice(self.pos - 1..self.pos + 1))
                } else {
                    Token::new(TokenType::Gt, self.slice_len(1))
                }
            }
            b'<' => {
                let peek = self.peek_char();
                if peek.is_some_and(|c| c == b'=') {
                    self.advance();
                    Token::new(TokenType::Lte, self.slice(self.pos - 1..self.pos + 1))
                } else {
                    Token::new(TokenType::Lt, self.slice_len(1))
                }
            }
            c if c.is_ascii_alphabetic() => {
                let ident = self.read_ident();
                Token::keyword(ident)
            }
            c if c.is_ascii_digit() => {
                let int = self.read_int();
                Token::new(TokenType::IntLit, int)
            }
            _ => Token::illegal(self.slice_len(1)),
        };

        self.advance();
        return Some(token);
    }
}

mod pred {
    pub fn is_valid_ident(c: u8) -> bool {
        return c == b'_' || c.is_ascii_alphanumeric();
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::TokenType;

    use super::Lexer;

    #[test]
    fn basic() {
        let code = "+-*/.,:;=";
        let mut lex = Lexer::from(code);

        let mut tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Plus));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Minus));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Star));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Slash));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Dot));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Comma));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Colon));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Semicolon));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Assign));

        tok = lex.next();
        assert!(tok.is_none());
    }

    #[test]
    fn comparison() {
        let code = "= ! != == > >= < <=";
        let mut lex = Lexer::from(code);

        let mut tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Assign));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Bang));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Neq));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Eq));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Gt));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Gte));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Lt));

        tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Lte));

        tok = lex.next();
        assert!(tok.is_none());
    }

    #[test]
    fn ident() {
        let code = "abubadibaba some_ident12";
        let mut lex = Lexer::from(code);

        let tok = lex.next();
        assert!(tok.is_some());

        let tok = tok.unwrap();
        assert_eq!(tok.ty, TokenType::Ident);
        assert_eq!(tok.lit, b"abubadibaba");

        let tok = lex.next();
        assert!(tok.is_some());

        let tok = tok.unwrap();
        assert_eq!(tok.ty, TokenType::Ident);
        assert_eq!(tok.lit, b"some_ident12");

        assert!(lex.next().is_none());
    }

    #[test]
    fn keywords() {
        let code = "let fn";
        let mut lex = Lexer::from(code);

        let tok = lex.next();
        assert!(tok.is_some());
        assert_eq!(tok.unwrap().ty, TokenType::Let);

        let tok = lex.next();
        assert!(tok.is_some_and(|t| t.ty == TokenType::Fn));
        assert!(lex.next().is_none());
    }

    #[test]
    fn literals() {
        let code = "123 999 0717";
        let mut lex = Lexer::from(code);

        let tok = lex.next();
        assert!(tok.is_some());

        let tok = tok.unwrap();
        assert_eq!(tok.ty, TokenType::IntLit);
        assert_eq!(tok.lit, b"123");

        let tok = lex.next();
        assert!(tok.is_some());

        let tok = tok.unwrap();
        assert_eq!(tok.ty, TokenType::IntLit);
        assert_eq!(tok.lit, b"999");
        
        let tok = lex.next();
        assert!(tok.is_some());

        let tok = tok.unwrap();
        assert_eq!(tok.ty, TokenType::IntLit);
        assert_eq!(tok.lit, b"0717");

        assert!(lex.next().is_none());
    }
}
