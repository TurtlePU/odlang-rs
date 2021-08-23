use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::{prelude::*, syntax::*};

pub fn typeck(term: Term) -> Result<(), TypeckErrors> {
    let MultiResult { result: _, collect } =
        Typeck::default().typeck_term(term);
    if collect.is_empty() {
        Ok(())
    } else {
        Err(collect)
    }
}

pub fn subst_type(body: Type, with: Type, what: Var) -> Type {
    match (*body).clone() {
        TyUnit => body,
        TyHole => body,
        TyVar(var) if var == what => with,
        TyVar(_) => body,
        TyArrow(from, to) => ty::arr(
            subst_type(from, with.clone(), what),
            subst_type(to, with, what),
        ),
        TyForall(n, x) => ty::forall(n, subst_type(x, with, what)),
        TyError => unreachable!(),
    }
}

pub type TypeckErrors = VecDeque<TypeckError>;

impl Named for TypeckErrors {
    fn pprint(&self, names: &Names) -> String {
        self.iter().map(|err| err.pprint(names)).join("\n")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeckError {
    NotAFunction(Type),
    NotAForall(Type),
    NotEqual(Type, Type),
}

use TypeckError::*;

impl Named for TypeckError {
    fn pprint(&self, names: &Names) -> String {
        match self {
            NotEqual(a, b) => format!(
                "Types should be equal: '{}', '{}'",
                a.pprint(names),
                b.pprint(names)
            ),
            NotAFunction(f) => {
                format!("Must be a function: '{}'", f.pprint(names))
            }
            NotAForall(f) => {
                format!("Must be a forall: '{}'", f.pprint(names))
            }
        }
    }
}

#[derive(Default)]
struct Typeck(HashMap<Var, Type>, AlphaGen);

type TypeckResult = MultiResult<Type, VecDeque<TypeckError>>;

impl Typeck {
    fn typeck_term(&mut self, term: Term) -> TypeckResult {
        match (*term).clone() {
            TmUnit => ty::unit().into(),
            TmVar(v) => self.get_or_alpha(v).into(),
            TmAbs(v, t, y) => self
                .insert(v, t.clone())
                .typeck_term(y)
                .map(move |y| ty::arr(t, y)),
            TmApp(f, x) => (self.typeck_term(f) + self.typeck_term(x))
                .then(|(f, x)| assert_app(f, x)),
            TmTyAbs(n, x) => self.typeck_term(x).map(move |x| ty::forall(n, x)),
            TmTyApp(f, t) => {
                self.typeck_term(f).then(move |f| assert_ty_app(f, t))
            }
            TmError => unreachable!(),
        }
    }

    fn get_or_alpha(&self, v: Var) -> Type {
        self.0.get(&v).cloned().unwrap_or_else(ty::hole)
    }

    fn insert(&mut self, v: Var, t: Type) -> &mut Self {
        self.0.insert(v, t);
        self
    }
}

fn assert_app(fun: Type, arg: Type) -> TypeckResult {
    match (*fun).clone() {
        TyArrow(from, to) if from == arg => to.into(),
        TyArrow(from, to) => TypeckResult::new(to, NotEqual(from, arg)),
        _ => TypeckResult::item(NotAFunction(fun)),
    }
}

fn assert_ty_app(fun: Type, arg: Type) -> TypeckResult {
    match (*fun).clone() {
        TyForall(var, inner) => subst_type(inner, arg, var).into(),
        _ => TypeckResult::item(NotAForall(fun)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_typeck() {
        assert_eq!(typeck(de::abs(0, ty::unit(), de::var(0))), Ok(()));
    }
}
