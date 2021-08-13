use std::rc::Rc;

use crate::{
    ident::Alpha,
    names::{Named, Names, Var},
};

pub type Term = Rc<TermData>;
pub type Type = Rc<TypeData>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TermData {
    TmUnit,
    TmVar(Var),
    TmAbs(Var, Type, Term),
    TmApp(Term, Term),
    TmTyAbs(Var, Term),
    TmTyApp(Term, Type),
    TmError(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeData {
    TyUnit,
    TyAlpha(Alpha),
    TyVar(Var),
    TyArrow(Type, Type),
    TyForall(Var, Type),
    TyError(String),
}

impl Named for Term {
    fn pprint(&self, names: &Names) -> String {
        use TermData::*;
        match (**self).clone() {
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
            TmError(err) => err,
        }
    }
}

impl Named for Type {
    fn pprint(&self, names: &Names) -> String {
        use TypeData::*;
        match (**self).clone() {
            TyUnit => "()".into(),
            TyAlpha(alp) => format!("{}", alp),
            TyVar(var) => names[var].clone(),
            TyArrow(f, t) => match *f {
                TyUnit | TyAlpha(_) | TyVar(_) => {
                    format!(
                        "{} -> {}",
                        f.pprint(names),
                        t.pprint(names)
                    )
                }
                _ => {
                    format!(
                        "({}) -> {}",
                        f.pprint(names),
                        t.pprint(names)
                    )
                }
            },
            TyForall(n, y) => {
                format!("/\\ {} => {}", names[n], y.pprint(names))
            }
            TyError(err) => err,
        }
    }
}

pub mod de {
    use super::{Term, TermData::*, Type};
    use crate::names::Var;

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

    pub fn error(name: impl Into<String>) -> Term {
        TmError(name.into()).into()
    }

    pub mod ty {
        use crate::{
            ident::Alpha,
            names::Var,
            term::{Type, TypeData::*},
        };

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

        pub fn error(name: impl Into<String>) -> Type {
            TyError(name.into()).into()
        }
    }
}
