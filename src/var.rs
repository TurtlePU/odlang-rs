#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Var {
    Free(String),
    Bound(usize),
}
