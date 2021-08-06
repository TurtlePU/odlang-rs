use std::{collections::HashMap, fmt::Display};

use crate::intern::{de::ty, Term, TermData::*, Type, TypeData::*, Var};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TypeckError {
    #[error("Not a function: '{0:?}'")]
    NotAFunction(Type),
    #[error("Not a type abstraction: '{0:?}'")]
    NotAForall(Type),
    #[error("Types must be equal: '{0:?}', '{1:?}'")]
    NotEqual(Type, Type),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub struct TypeckResult {
    pub ty: Type,
    pub errors: Vec<TypeckError>,
}

impl From<Type> for TypeckResult {
    fn from(ty: Type) -> Self {
        Self { ty, errors: vec![] }
    }
}

impl Display for TypeckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\n", &self.ty)?;
        for err in &self.errors {
            write!(f, "{}\n", err)?;
        }
        Ok(())
    }
}

impl TypeckResult {
    fn map(mut self, f: impl FnOnce(Type) -> Type) -> Self {
        self.ty = f(self.ty);
        self
    }

    fn then(self, f: impl FnOnce(Type) -> Self) -> Self {
        let Self { ty, mut errors } = self;
        let Self { ty, errors: mut new_errors } = f(ty);
        errors.append(&mut new_errors);
        Self { ty, errors }
    }
}

pub fn typeck(term: Term) -> TypeckResult {
    Typeck::default().typeck(term)
}

#[derive(Default)]
struct Typeck(HashMap<Var, Type>);

impl Typeck {
    fn typeck(&mut self, term: Term) -> TypeckResult {
        match (*term).clone() {
            TmUnit => ty::unit().into(),
            TmVar(v) => self.get(v).into(),
            TmAbs(v, t, y) => self.insert(v, t.clone()).typeck(y).map(|y| ty::arr(t, y)),
            TmApp(f, x) => self.typeck(f).then(|f| self.typeck(x).then(|x| assert_app(f, x))),
            TmTyAbs(n, x) => self.typeck(x).map(|x| ty::forall(n, x)),
            TmTyApp(f, t) => self.typeck(f).then(|f| assert_ty_app(f, t)),
        }
    }

    fn get(&mut self, v: Var) -> Type {
        self.0.entry(v).or_insert(ty::hole()).clone()
    }

    fn insert(&mut self, v: Var, t: Type) -> &mut Self {
        self.0.insert(v, t);
        self
    }
}

fn assert_app(fun: Type, arg: Type) -> TypeckResult {
    use TypeckError::*;
    match (*fun).clone() {
        TyArrow(from, to) if from == arg => to.into(),
        TyArrow(from, to) => TypeckResult {
            ty: to,
            errors: vec![NotEqual(from, arg)],
        },
        _ => TypeckResult {
            ty: ty::hole(),
            errors: vec![NotAFunction(fun)],
        },
    }
}

fn assert_ty_app(fun: Type, arg: Type) -> TypeckResult {
    use TypeckError::*;
    match (*fun).clone() {
        TyForall(var, inner) => subst_type(inner, arg, var).into(),
        _ => TypeckResult {
            ty: ty::hole(),
            errors: vec![NotAForall(fun)],
        },
    }
}

pub fn subst_type(body: Type, with: Type, what: Var) -> Type {
    match (*body).clone() {
        TyUnit => ty::unit(),
        TyHole => ty::hole(),
        TyVar(var) if var == what => with,
        TyVar(other) => ty::var(other),
        TyArrow(from, to) => ty::arr(
            subst_type(from, with.clone(), what),
            subst_type(to, with, what),
        ),
        TyForall(n, x) => ty::forall(n, subst_type(x, with, what)),
    }
}

#[cfg(test)]
mod tests {
    use super::typeck;
    use crate::intern::de::{self, ty};

    #[test]
    fn simple_typeck() {
        assert_eq!(
            typeck(de::abs(0, ty::unit(), de::var(0))),
            ty::arr(ty::unit(), ty::unit()).into()
        );
    }
}
