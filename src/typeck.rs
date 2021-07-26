use crate::bruijn::{DeBruijnTerm, Var};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    TyUnit,
    TyHole,
    TyArrow(Box<Type>, Box<Type>),
}

pub mod ty {
    use super::Type;

    pub fn unit() -> Type {
        Type::TyUnit
    }

    pub fn hole() -> Type {
        Type::TyHole
    }

    pub fn arr(from: Type, to: Type) -> Type {
        Type::TyArrow(Box::new(from), Box::new(to))
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TypeckError {
    #[error("Not a function: '{0:?}'")]
    NotAFunction(Type),
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
            TmApp(f, x) => assert_app(self.typeck(*f)?, self.typeck(*x)?)
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

#[cfg(test)]
mod tests {
    use super::{typeck, ty};
    use crate::bruijn::de;

    #[test]
    fn simple_typeck() {
        assert_eq!(
            typeck(de::abs(ty::unit(), de::var(0))),
            Ok(ty::arr(ty::unit(), ty::unit()))
        );
    }
}
