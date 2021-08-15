use std::collections::VecDeque;

use crate::multi_result::MultiResult;

use super::{error::ParseError, InputTerm};

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> ParseResult<InputTerm> {
        todo!()
    }
}

type ParseResult<T> = MultiResult<T, (), VecDeque<ParseError>>;
