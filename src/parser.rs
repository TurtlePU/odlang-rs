#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub from: Position,
    pub to: Position,
}

#[derive(Debug, Clone)]
pub enum InputTerm {
    TmUnit,
    TmVar(String),
    TmAbs(String, InputType, Box<InputTerm>),
    TmApp(Box<InputTerm>, Box<InputTerm>),
    TmTyAbs(String, Box<InputTerm>),
    TmTyApp(Box<InputTerm>, InputType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    TyUnit,
    TyHole,
    TyVar(String),
    TyArrow(Box<InputType>, Box<InputType>),
    TyForall(String, Box<InputType>),
}

pub fn parse(text: &str) -> Result<InputTerm, String> {
    todo!()
}

pub mod inp {
    use super::{InputTerm, InputType};

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

    pub fn tyabs(par: impl Into<String>, body: InputTerm) -> InputTerm {
        InputTerm::TmTyAbs(par.into(), Box::new(body))
    }

    pub fn app(f: InputTerm, x: InputTerm) -> InputTerm {
        InputTerm::TmApp(Box::new(f), Box::new(x))
    }

    pub fn tyapp(f: InputTerm, x: InputType) -> InputTerm {
        InputTerm::TmTyApp(Box::new(f), x)
    }

    pub mod ty {
        use super::InputType;

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
    }
}
