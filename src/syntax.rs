use std::{ops::Deref, rc::Rc};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term(Rc<TermData>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type(Rc<TypeData>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TermData {
    TmUnit,
    TmVar(Var),
    TmAbs(Var, Type, Term),
    TmApp(Term, Term),
    TmTyAbs(Var, Term),
    TmTyApp(Term, Type),
    TmError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeData {
    TyUnit,
    TyHole,
    TyVar(Var),
    TyArrow(Type, Type),
    TyForall(Var, Type),
    TyError,
}

pub use TermData::*;
pub use TypeData::*;

pub mod de {
    use super::*;

    pub fn unit() -> Term {
        TmUnit.into()
    }

    pub fn abs(
        param: impl Into<Var>,
        r#type: impl Into<Type>,
        body: impl Into<Term>,
    ) -> Term {
        TmAbs(param.into(), r#type.into(), body.into()).into()
    }

    pub fn app(f: impl Into<Term>, x: impl Into<Term>) -> Term {
        TmApp(f.into(), x.into()).into()
    }

    pub fn var(key: impl Into<Var>) -> Term {
        TmVar(key.into()).into()
    }

    pub fn ty_abs(param: impl Into<Var>, body: impl Into<Term>) -> Term {
        TmTyAbs(param.into(), body.into()).into()
    }

    pub fn ty_app(f: impl Into<Term>, ty: impl Into<Type>) -> Term {
        TmTyApp(f.into(), ty.into()).into()
    }

    pub fn error() -> Term {
        TmError.into()
    }
}

pub mod ty {
    use super::*;

    pub fn unit() -> Type {
        TyUnit.into()
    }

    pub fn hole() -> Type {
        TyHole.into()
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
}

impl From<Var> for Term {
    fn from(var: Var) -> Self {
        de::var(var)
    }
}

impl From<TermData> for Term {
    fn from(data: TermData) -> Self {
        Self(data.into())
    }
}

impl Default for Term {
    fn default() -> Self {
        de::error()
    }
}

impl Deref for Term {
    type Target = TermData;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Named for Term {
    fn pprint(&self, names: &Names) -> String {
        match (*self.0).clone() {
            TmUnit => "()".into(),
            TmVar(var) => names[var].clone(),
            TmAbs(n, t, y) => {
                format!(
                    "\\{}: {}. {}",
                    names[n],
                    t.pprint(names),
                    y.pprint(names)
                )
            }
            TmApp(f, x) => match *f {
                TmAbs(_, _, _) => {
                    format!("({}) {}", f.pprint(names), x.pprint(names))
                }
                _ => format!("{} {}", f.pprint(names), x.pprint(names)),
            },
            TmTyAbs(n, y) => format!("/\\ {}. {}", names[n], y.pprint(names)),
            TmTyApp(f, x) => match *f {
                TmTyAbs(_, _) => {
                    format!("({}) [{}]", f.pprint(names), x.pprint(names))
                }
                _ => {
                    format!("{} [{}]", f.pprint(names), x.pprint(names))
                }
            },
            TmError => "ERROR".into(),
        }
    }
}

impl From<Var> for Type {
    fn from(var: Var) -> Self {
        ty::var(var)
    }
}

impl From<TypeData> for Type {
    fn from(data: TypeData) -> Self {
        Self(data.into())
    }
}

impl Default for Type {
    fn default() -> Self {
        ty::error()
    }
}

impl Deref for Type {
    type Target = TypeData;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Named for Type {
    fn pprint(&self, names: &Names) -> String {
        match (**self).clone() {
            TyUnit => "()".into(),
            TyHole => "_".into(),
            TyVar(var) => names[var].clone(),
            TyArrow(f, t) => match *f {
                TyUnit | TyHole | TyVar(_) => {
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
