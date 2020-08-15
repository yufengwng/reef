//! lex

use crate::ast::TnKind;
use crate::ast::Token;

pub struct Cursor<'a> {
    src: &'a [u8],
    len: usize,
    idx: usize,
    row: usize,
}

impl Cursor<'_> {
    pub fn next_token(&mut self) -> Option<Token> {
        if self.is_eof() {
            return None;
        }
        match self.curr() {
            b' ' | b'\t' | b'\r' | b'#' => None,
            b'\n' => {
                let token = Token::sep(self.line, self.idx);
                self.bump();
                self.bump_line();
                token
            }
            b';' => {
                let token = Token::sep(self.line, self.idx);
                self.bump();
                token
            }
            _ => self.next_word(),
        }
    }

    pub fn next_word(&mut self) -> Token {
        let line = self.row;
        let start = self.idx;
        let mut has_escape = false;
        while !self.is_eof() {}
    }

    pub fn eat_line_space(&mut self) {
        while !self.is_eof() {
            match self.curr() {
                b' ' | b'\t' | b'\r' => self.bump(),
                _ => break,
            }
        }
    }

    pub fn eat_line_comment(&mut self) {
        while !self.is_eof() {
            match self.curr() {
                b'#' => self.eat_to_newline(),
                _ => break,
            }
        }
    }

    pub fn eat_line_noise(&mut self) {
        while !self.is_eof() {
            match self.curr() {
                b' ' | b'\t' | b'\r' => self.bump(),
                b'#' => self.eat_to_newline(),
                _ => break,
            }
        }
    }

    pub fn eat_to_newline(&mut self) {
        while !self.is_eof() && self.curr() != b'\n' {
            self.bump();
        }
    }

    fn is_eof(&self) -> bool {
        self.idx >= self.len
    }

    fn curr(&self) -> u8 {
        self.src[self.idx]
    }

    fn peek(&self) -> u8 {
        if self.idx + 1 < self.len {
            self.src[self.idx + 1]
        } else {
            b'\0'
        }
    }

    fn bump(&mut self) {
        self.idx += 1;
    }

    fn bump_line(&mut self) {
        self.row += 1;
    }
}
