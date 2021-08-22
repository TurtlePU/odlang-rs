use std::ops::Index;

#[derive(Default)]
pub struct Names(Vec<String>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Var(usize);

pub trait Named {
    fn pprint(&self, names: &Names) -> String;
}

impl From<usize> for Var {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl Index<Var> for Names {
    type Output = String;

    fn index(&self, index: Var) -> &Self::Output {
        &self.0[index.0]
    }
}

impl Names {
    pub fn push(&mut self, name: String) -> Var {
        self.0.push(name);
        Var(self.0.len() - 1)
    }
}
