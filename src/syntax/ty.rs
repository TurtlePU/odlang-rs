use std::rc::Rc;

pub use TypeData::*;

use crate::atoms::*;

pub type Type = Rc<TypeData>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeData {
    TyUnit,
    TyAlpha(Alpha),
    TyVar(Var),
    TyArrow(Type, Type),
    TyForall(Var, Type),
    TyError,
}

pub fn unit() -> Type {
    TyUnit.into()
}

pub fn hole(num: impl Into<Alpha>) -> Type {
    TyAlpha(num.into()).into()
}

pub fn var(key: impl Into<Var>) -> Type {
    TyVar(key.into()).into()
}

pub fn arr(from: impl Into<Type>, to: impl Into<Type>) -> Type {
    TyArrow(from.into(), to.into()).into()
}

pub fn forall(param: impl Into<Var>, of: impl Into<Type>) -> Type {
    TyForall(param.into(), of.into()).into()
}

pub fn error() -> Type {
    TyError.into()
}

impl Named for Type {
    fn pprint(&self, names: &Names) -> String {
        match (**self).clone() {
            TyUnit => "()".into(),
            TyAlpha(alp) => format!("{}", alp),
            TyVar(var) => names[var].clone(),
            TyArrow(f, t) => match *f {
                TyUnit | TyAlpha(_) | TyVar(_) => {
                    format!("{} -> {}", f.pprint(names), t.pprint(names))
                }
                _ => {
                    format!("({}) -> {}", f.pprint(names), t.pprint(names))
                }
            },
            TyForall(n, y) => {
                format!("/\\ {} => {}", names[n], y.pprint(names))
            }
            TyError => "ERROR".into(),
        }
    }
}
