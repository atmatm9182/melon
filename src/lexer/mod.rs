use std::ops::Range;

use self::token::{Token, TokenType};

mod token;

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
    fn is_at_end(&self) -> bool {
        self.pos >= self.contents.len()
    }

    #[inline(always)]
    fn cur_char(&self) -> u8 {
        self.contents[self.pos]
    }

    #[inline(always)]
    fn slice_len(&self, len: usize) -> &'a [u8] {
        self.slice(self.pos..self.pos + len)
    }

    #[inline(always)]
    fn slice(&self, range: Range<usize>) -> &'a [u8] {
        &self.contents[range]
    }

    #[inline(always)]
    fn chop(&mut self) -> u8 {
        let c = self.cur_char();
        self.advance();
        return c;
    }

    fn read_ident(&mut self) -> &'a [u8] {
        let start = self.pos;
        while !self.is_at_end() {
            let c = self.chop();
            if !pred::is_valid_ident(c) {
                break;
            }
        }

        self.slice(start..self.pos)
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
            pos: 0
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

        let token = Some(match self.cur_char() {
            b'.' => Token::new(TokenType::Dot, self.slice_len(1)),
            b',' => Token::new(TokenType::Comma, self.slice_len(1)),
            b':' => Token::new(TokenType::Colon, self.slice_len(1)),
            b';' => Token::new(TokenType::Semicolon, self.slice_len(1)),
            b'+' => Token::new(TokenType::Plus, self.slice_len(1)),
            b'-' => Token::new(TokenType::Minus, self.slice_len(1)),
            b'*' => Token::new(TokenType::Star, self.slice_len(1)),
            b'/' => Token::new(TokenType::Slash, self.slice_len(1)),
            b'=' => Token::new(TokenType::Assign, self.slice_len(1)),
            c if c.is_ascii_alphabetic() => {
                let ident = self.read_ident();
                Token::new(TokenType::Ident, ident)
            }
            _ => Token::illegal(self.slice_len(1)),
        });

        self.advance();
        return token;
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
    fn ident() {
        let code = "abubadibaba";
        let mut lex = Lexer::from(code);

        let tok = lex.next();
        assert!(tok.is_some());

        let tok = tok.unwrap();
        assert!(tok.ty == TokenType::Ident);
        assert!(tok.lit == code.as_bytes());
    }
}
