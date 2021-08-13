use std::{collections::HashSet, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Unbound(pub HashSet<String>);

impl Error for Unbound {}

impl Display for Unbound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for name in &self.0 {
            writeln!(f, "Unbound name: {}", name)?;
        }
        Ok(())
    }
}
