use std::rc::Rc;

pub use TermData::*;

use crate::atoms::*;

use super::Type;

pub type Term = Rc<TermData>;

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

impl Named for Term {
    fn pprint(&self, names: &Names) -> String {
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
