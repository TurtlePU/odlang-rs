use std::{collections::HashSet, error::Error, fmt::Display};

#[derive(Debug, Default)]
pub struct Unbound(HashSet<String>);

impl Error for Unbound {}

impl Display for Unbound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for name in &self.0 {
            writeln!(f, "Unbound name: {}", name)?;
        }
        Ok(())
    }
}

impl Unbound {
    pub fn report(&mut self, unbound: String) {
        self.0.insert(unbound);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
