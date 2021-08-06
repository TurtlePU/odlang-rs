use std::{collections::HashMap, ops::Add};

use crate::intern::{de::ty, Term, TermData::*, Type, TypeData::*, Var};

#[derive(Debug, PartialEq, Eq)]
pub enum TypeckError {
    NotAFunction(Type),
    NotAForall(Type),
    NotEqual(Type, Type),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeckResult<T = Type>(pub T, pub Vec<TypeckError>);

impl<T> From<T> for TypeckResult<T> {
    fn from(value: T) -> Self {
        Self(value, vec![])
    }
}

impl<T> TypeckResult<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> TypeckResult<U> {
        TypeckResult(f(self.0), self.1)
    }

    fn then<U>(self, f: impl FnOnce(T) -> TypeckResult<U>) -> TypeckResult<U> {
        let TypeckResult(x, mut errors) = self;
        let TypeckResult(x, mut new_errors) = f(x);
        errors.append(&mut new_errors);
        TypeckResult(x, errors)
    }
}

impl<T, U> Add<TypeckResult<U>> for TypeckResult<T> {
    type Output = TypeckResult<(T, U)>;

    fn add(self, rhs: TypeckResult<U>) -> Self::Output {
        self.then(|x| rhs.map(|y| (x, y)))
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
            TmAbs(v, t, y) => {
                self.insert(v, t.clone()).typeck(y).map(|y| ty::arr(t, y))
            },
            TmApp(f, x) => (self.typeck(f) + self.typeck(x)).then(|(f, x)| {
                assert_app(f, x)
            }),
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
        TyArrow(from, to) => TypeckResult(to, vec![NotEqual(from, arg)]),
        _ => TypeckResult(ty::hole(), vec![NotAFunction(fun)]),
    }
}

fn assert_ty_app(fun: Type, arg: Type) -> TypeckResult {
    use TypeckError::*;
    match (*fun).clone() {
        TyForall(var, inner) => subst_type(inner, arg, var).into(),
        _ => TypeckResult(ty::hole(), vec![NotAForall(fun)]),
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
