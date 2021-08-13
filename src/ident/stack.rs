use std::collections::HashMap;

use crate::atoms::Var;

#[derive(Default)]
pub struct Stack(HashMap<String, Vec<Var>>);

impl Stack {
    pub fn push(&mut self, name: String, var: Var) {
        self.0.entry(name).or_default().push(var)
    }

    pub fn map(&self, name: &String) -> Option<Var> {
        self.0.get(name)?.last().copied()
    }

    pub fn pop(&mut self, name: &String) -> Option<Var> {
        self.0.get_mut(name)?.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.0.values().all(|stack| stack.is_empty())
    }
}
