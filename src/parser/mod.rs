pub mod inp;
pub mod ty;
mod state;
mod error;

pub use inp::*;
pub use ty::*;

use self::{error::ParseErrors, state::Parser};

pub fn parse(text: &str) -> Result<InputTerm, ParseErrors> {
    let res: Result<_, _> = Parser::from(text).parse().into();
    res.map_err(ParseErrors::from)
}
