use crate::atoms::Var;

#[derive(Default)]
pub struct Stack<'a>(Option<(&'a Self, String, Var)>);

impl<'a> Stack<'a> {
    pub fn push(&'a self, name: String, var: Var) -> Self {
        Self(Some((self, name, var)))
    }

    pub fn map(&self, name: String) -> Result<Var, String> {
        match self.0 {
            Some((_, ref key, var)) if *key == name => Ok(var),
            Some((prev, _, _)) => prev.map(name),
            None => Err(name),
        }
    }
}
