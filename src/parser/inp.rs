use crate::multi_result::ErrValue;

use super::InputType;

pub use InputTerm::*;

#[derive(Debug, Clone)]
pub enum InputTerm {
    TmUnit,
    TmVar(String),
    TmAbs(String, InputType, Box<InputTerm>),
    TmApp(Box<InputTerm>, Box<InputTerm>),
    TmTyAbs(String, Box<InputTerm>),
    TmTyApp(Box<InputTerm>, InputType),
    TmError,
}

pub fn unit() -> InputTerm {
    TmUnit
}

pub fn var(name: impl Into<String>) -> InputTerm {
    TmVar(name.into())
}

pub fn abs(
    par: impl Into<String>,
    ty: InputType,
    body: InputTerm,
) -> InputTerm {
    TmAbs(par.into(), ty, Box::new(body))
}

pub fn app(f: InputTerm, x: InputTerm) -> InputTerm {
    TmApp(Box::new(f), Box::new(x))
}

pub fn tyabs(par: impl Into<String>, body: InputTerm) -> InputTerm {
    TmTyAbs(par.into(), Box::new(body))
}

pub fn tyapp(f: InputTerm, x: InputType) -> InputTerm {
    TmTyApp(Box::new(f), x)
}

pub fn err() -> InputTerm {
    TmError
}

impl ErrValue for InputTerm {
    fn err_value() -> Self {
        err()
    }
}
