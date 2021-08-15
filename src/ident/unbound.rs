use std::{collections::HashSet, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Unbound(HashSet<String>);

impl From<HashSet<String>> for Unbound {
    fn from(errs: HashSet<String>) -> Self {
        Self(errs)
    }
}

impl Error for Unbound {}

impl Display for Unbound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for name in &self.0 {
            writeln!(f, "Unbound name: {}", name)?;
        }
        Ok(())
    }
}
