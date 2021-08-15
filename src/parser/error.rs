use std::{error::Error, fmt::Display};

use crate::atoms::Position;

#[derive(Debug)]
pub struct ParseErrors(Vec<ParseError>);

pub use ParseError::*;

impl<I> From<I> for ParseErrors
where
    I: IntoIterator<Item = ParseError>,
{
    fn from(errors: I) -> Self {
        Self(errors.into_iter().collect())
    }
}

impl Error for ParseErrors {}

impl Display for ParseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.0 {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnknownSymbol(char, Position),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownSymbol(c, p) => {
                writeln!(f, "[{}]: Unknown symbol '{}'", p, c)
            }
        }
    }
}
