use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParseErrors(pub Vec<ParseError>);

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
pub struct ParseError;

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
