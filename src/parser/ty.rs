pub use InputType::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    TyUnit,
    TyHole,
    TyVar(String),
    TyArrow(Box<InputType>, Box<InputType>),
    TyForall(String, Box<InputType>),
}

pub fn unit() -> InputType {
    InputType::TyUnit
}

pub fn hole() -> InputType {
    InputType::TyHole
}

pub fn var(name: impl Into<String>) -> InputType {
    InputType::TyVar(name.into())
}

pub fn arr(from: InputType, to: InputType) -> InputType {
    InputType::TyArrow(Box::new(from), Box::new(to))
}

pub fn forall(par: impl Into<String>, body: InputType) -> InputType {
    InputType::TyForall(par.into(), Box::new(body))
}
