//! parse

use crate::ast::Cmd;
use crate::lex::Cursor;

pub struct Parser<'a> {
    cur: Cursor<'a>,
}

impl Parser<'_> {
    fn parse_cmd(&mut self) -> Option<Cmd> {
    }
}
