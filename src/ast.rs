//! ast

pub enum TnKind {
    /// Command separator, either newline or semicolon
    SEP,
    /// An unexpected or erroneous token
    ERR,
    /// End-of-file
    EOF,
}

pub struct Token {
    kind: TnKind,
    line: usize,
    idx: usize,
    len: usize,
    txt: Option<String>,
}

impl Token {
    pub fn eof(line: usize, idx: usize) -> Token {
        Token {
            kind: TnKind::EOF,
            line,
            idx,
            len: 0,
        }
    }

    pub fn sep(line: usize, idx: usize) -> Token {
        Token {
            kind: TnKind::SEP,
            line,
            idx,
            len: 1,
        }
    }
}

pub struct Cmd {}
