use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum InputTerm {
    TmUnit(Range),
    TmVar(Range, String),
    TmAbs(Range, String, InputType, Box<InputTerm>),
    TmApp(Range, Box<InputTerm>, Box<InputTerm>),
    TmTyAbs(Range, String, Box<InputTerm>),
    TmTyApp(Range, Box<InputTerm>, InputType),
    TmError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    TyUnit(Range),
    TyHole(Range),
    TyVar(Range, String),
    TyArrow(Range, Box<InputType>, Box<InputType>),
    TyForall(Range, String, Box<InputType>),
    TyError,
}

pub use InputTerm::*;
pub use InputType::*;

pub mod inp {
    use super::*;

    pub fn unit(range: Range) -> InputTerm {
        TmUnit(range)
    }

    pub fn var(range: Range, name: impl Into<String>) -> InputTerm {
        TmVar(range, name.into())
    }

    pub fn abs(
        range: Range,
        par: impl Into<String>,
        ty: InputType,
        body: InputTerm,
    ) -> InputTerm {
        TmAbs(range, par.into(), ty, Box::new(body))
    }

    pub fn app(range: Range, f: InputTerm, x: InputTerm) -> InputTerm {
        TmApp(range, Box::new(f), Box::new(x))
    }

    pub fn tyabs(
        range: Range,
        par: impl Into<String>,
        body: InputTerm,
    ) -> InputTerm {
        TmTyAbs(range, par.into(), Box::new(body))
    }

    pub fn tyapp(range: Range, f: InputTerm, x: InputType) -> InputTerm {
        TmTyApp(range, Box::new(f), x)
    }

    pub fn err() -> InputTerm {
        TmError
    }
}

pub mod ty {
    use super::*;

    pub fn unit(range: Range) -> InputType {
        TyUnit(range)
    }

    pub fn hole(range: Range) -> InputType {
        TyHole(range)
    }

    pub fn var(range: Range, name: impl Into<String>) -> InputType {
        TyVar(range, name.into())
    }

    pub fn arr(range: Range, from: InputType, to: InputType) -> InputType {
        TyArrow(range, Box::new(from), Box::new(to))
    }

    pub fn forall(
        range: Range,
        par: impl Into<String>,
        body: InputType,
    ) -> InputType {
        TyForall(range, par.into(), Box::new(body))
    }

    pub fn error() -> InputType {
        TyError
    }
}

impl ErrValue for InputTerm {
    fn err_value() -> Self {
        inp::err()
    }
}

impl ErrValue for InputType {
    fn err_value() -> Self {
        ty::error()
    }
}
