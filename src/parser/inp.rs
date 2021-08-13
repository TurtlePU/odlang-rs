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
}

pub fn unit() -> InputTerm {
    InputTerm::TmUnit
}

pub fn var(name: impl Into<String>) -> InputTerm {
    InputTerm::TmVar(name.into())
}

pub fn abs(
    par: impl Into<String>,
    ty: InputType,
    body: InputTerm,
) -> InputTerm {
    InputTerm::TmAbs(par.into(), ty, Box::new(body))
}

pub fn app(f: InputTerm, x: InputTerm) -> InputTerm {
    InputTerm::TmApp(Box::new(f), Box::new(x))
}

pub fn tyabs(par: impl Into<String>, body: InputTerm) -> InputTerm {
    InputTerm::TmTyAbs(par.into(), Box::new(body))
}

pub fn tyapp(f: InputTerm, x: InputType) -> InputTerm {
    InputTerm::TmTyApp(Box::new(f), x)
}
