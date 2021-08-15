use std::collections::VecDeque;

use itertools::Itertools;

use crate::{atoms::{Named, Names}, syntax::Type};

pub use TypeckError::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeckError {
    NotAFunction(Type),
    NotAForall(Type),
    NotEqual(Type, Type),
}

impl Named for VecDeque<TypeckError> {
    fn pprint(&self, names: &Names) -> String {
        self.iter().map(|err| err.pprint(names)).join("\n")
    }
}

impl Named for TypeckError {
    fn pprint(&self, names: &Names) -> String {
        use TypeckError::*;
        match self {
            NotEqual(a, b) => format!(
                "Types should be equal: '{}', '{}'",
                a.pprint(names),
                b.pprint(names)
            ),
            NotAFunction(f) => {
                format!("Must be a function: '{}'", f.pprint(names))
            }
            NotAForall(f) => {
                format!("Must be a forall: '{}'", f.pprint(names))
            }
        }
    }
}
