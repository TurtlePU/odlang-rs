use crate::{
    bruijn::{de::ty, DeBruijnTerm, Type},
    var::Var,
};
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

pub fn typeck(term: DeBruijnTerm) -> Result<Type, TypeckError> {
    Typeck::default().typeck(term)
}

#[derive(Default)]
struct Typeck(Vec<Type>);

impl Typeck {
    fn typeck(&mut self, term: DeBruijnTerm) -> Result<Type, TypeckError> {
        use DeBruijnTerm::*;
        use Var::*;
        match term {
            TmUnit => Ok(ty::unit()),
            TmVar(Free(_)) => Ok(ty::hole()),
            TmVar(Bound(i)) => Ok(self.get(i)),
            TmAbs(t, y) => {
                self.0.push(t.clone());
                let ytype = self.typeck(*y)?;
                self.0.pop();
                Ok(ty::arr(t, ytype))
            }
            TmApp(f, x) => assert_app(self.typeck(*f)?, self.typeck(*x)?),
            TmTyAbs(x) => self.typeck(*x).map(ty::forall),
            TmTyApp(f, t) => assert_ty_app(self.typeck(*f)?, t),
        }
    }

    fn get(&self, i: usize) -> Type {
        self.0[self.0.len() - i - 1].clone()
    }
}

fn assert_app(fun: Type, arg: Type) -> Result<Type, TypeckError> {
    use Type::*;
    use TypeckError::*;
    match fun {
        TyArrow(from, to) if *from == arg => Ok((*to).clone()),
        TyArrow(from, _) => Err(NotEqual((*from).clone(), arg.clone())),
        _ => Err(NotAFunction(fun.clone())),
    }
}

fn assert_ty_app(fun: Type, arg: Type) -> Result<Type, TypeckError> {
    use Type::*;
    use TypeckError::*;
    match fun {
        TyForall(inner) => Ok(unshift_type(subst_type(*inner, shift_type(arg, 0), 0), 0)),
        _ => Err(NotAForall(fun.clone())),
    }
}

pub fn shift_type(body: Type, thr: usize) -> Type {
    use Type::*;
    use Var::*;
    match body {
        TyUnit => ty::unit(),
        TyHole => ty::hole(),
        TyVar(Bound(i)) if i >= thr => ty::var(i + 1),
        TyVar(free) => ty::var(free),
        TyArrow(from, to) => ty::arr(shift_type(*from, thr), shift_type(*to, thr)),
        TyForall(x) => ty::forall(shift_type(*x, thr + 1)),
    }
}

pub fn unshift_type(body: Type, thr: usize) -> Type {
    use Type::*;
    use Var::*;
    match body {
        TyUnit => ty::unit(),
        TyHole => ty::hole(),
        TyVar(Bound(i)) if i >= thr => ty::var(i - 1),
        TyVar(free) => ty::var(free),
        TyArrow(from, to) => ty::arr(unshift_type(*from, thr), unshift_type(*to, thr)),
        TyForall(x) => ty::forall(unshift_type(*x, thr + 1)),
    }
}

pub fn subst_type(body: Type, with: Type, depth: usize) -> Type {
    use Type::*;
    use Var::*;
    match body {
        TyUnit => ty::unit(),
        TyHole => ty::hole(),
        TyVar(Bound(i)) if i == depth => with,
        TyVar(other) => ty::var(other),
        TyArrow(from, to) => ty::arr(
            subst_type(*from, with.clone(), depth),
            subst_type(*to, with.clone(), depth),
        ),
        TyForall(x) => ty::forall(subst_type(*x, with, depth + 1)),
    }
}

#[cfg(test)]
mod tests {
    use super::typeck;
    use crate::bruijn::de::{self, ty};

    #[test]
    fn simple_typeck() {
        assert_eq!(
            typeck(de::abs(ty::unit(), de::var(0))),
            Ok(ty::arr(ty::unit(), ty::unit()))
        );
    }
}
