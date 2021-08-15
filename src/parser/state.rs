use std::collections::VecDeque;

use crate::multi_result::MultiResult;

use super::{error::ParseError, InputTerm};

type ParseResult<T> = MultiResult<T, (), VecDeque<ParseError>>;

pub fn parse_term(input: &str) -> ParseResult<InputTerm> {
    todo!()
}
