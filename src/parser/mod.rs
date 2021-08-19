pub mod error;
pub mod inp;
pub mod ty;
mod tokens;

use std::collections::VecDeque;

pub use error::*;
pub use inp::*;
pub use ty::*;

use crate::multi_result::MultiResult;

use self::tokens::{Token, Tokenizer};

use itertools::Itertools;

pub fn parse(text: &str) -> Result<InputTerm, ParseErrors> {
    let (tokens, mut errs): (Vec<_>, VecDeque<_>) =
        Tokenizer::from(text).partition_result();
    let ParseResult {
        result,
        state: _,
        mut errors,
    } = Parser::from(&tokens[..]).parse();
    errs.append(&mut errors);
    if errs.is_empty() {
        Ok(result)
    } else {
        Err(errs.into())
    }
}

#[derive(Clone, Copy)]
struct Parser<'a> {
    tokens: &'a [Token<'a>],
    position: usize,
}

impl<'a> From<&'a [Token<'a>]> for Parser<'a> {
    fn from(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
}

type ParseResult<T> = MultiResult<T, (), VecDeque<ParseError>>;

impl<'a> Parser<'a> {
    fn parse(mut self) -> ParseResult<InputTerm> {
        let result = self.parse_app(0);
        assert!(self.position == self.tokens.len());
        result
    }

    fn parse_app(&mut self, indent: usize) -> ParseResult<InputTerm> {
        todo!()
    }
}
