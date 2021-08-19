pub use InputType::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    TyUnit,
    TyHole,
    TyVar(String),
    TyArrow(Box<InputType>, Box<InputType>),
    TyForall(String, Box<InputType>),
    TyError,
}

pub fn unit() -> InputType {
    TyUnit
}

pub fn hole() -> InputType {
    TyHole
}

pub fn var(name: impl Into<String>) -> InputType {
    TyVar(name.into())
}

pub fn arr(from: InputType, to: InputType) -> InputType {
    TyArrow(Box::new(from), Box::new(to))
}

pub fn forall(par: impl Into<String>, body: InputType) -> InputType {
    TyForall(par.into(), Box::new(body))
}

pub fn error() -> InputType {
    TyError
}
