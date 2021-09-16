use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum InputTermRec<Rec, Type> {
    TmUnit,
    TmVar(String),
    TmAbs(String, Type, Rec),
    TmApp(Rec, Rec),
    TmTyAbs(String, Rec),
    TmTyApp(Rec, Type),
}

pub use InputTermRec::*;

#[derive(Debug, Clone)]
pub struct InputTerm(pub InputTermRec<Box<InputTerm>, InputType>, pub Range);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputTypeRec<Rec> {
    TyUnit,
    TyHole,
    TyVar(String),
    TyArrow(Rec, Rec),
    TyForall(String, Rec),
}

pub use InputTypeRec::*;

#[derive(Debug, Clone)]
pub struct InputType(pub InputTypeRec<Box<InputType>>, pub Range);
