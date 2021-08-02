use std::rc::Rc;

use crate::parser::{InputTerm, InputType};

pub type Term = Rc<TermData>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TermData {
    TmUnit,
    TmVar(Var),
    TmAbs(String, Type, Term),
    TmApp(Term, Term),
    TmTyAbs(String, Term),
    TmTyApp(Term, Type),
}

pub type Type = Rc<TypeData>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeData {
    TyUnit,
    TyHole,
    TyVar(Var),
    TyArrow(Type, Type),
    TyForall(String, Type),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Var {
    Free(String),
    Bound(usize, String),
}

pub fn de_bruijn(term: InputTerm) -> Term {
    Context(None).rename_term(term)
}

struct Context<'a>(Option<(&'a Context<'a>, Either, String)>);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Either {
    Term,
    Type,
}

impl<'a> Context<'a> {
    fn rename_term(&'a self, term: InputTerm) -> Term {
        use Either::*;
        use InputTerm::*;
        match term {
            TmUnit => de::unit(),
            TmVar(x) => de::var(self.find(Term, x)),
            TmApp(f, x) => de::app(self.rename_term(*f), self.rename_term(*x)),
            TmAbs(x, t, y) => de::abs(
                x.clone(),
                self.rename_type(t),
                self.with(Term, x).rename_term(*y),
            ),
            TmTyAbs(t, x) => {
                de::ty_abs(t.clone(), self.with(Type, t).rename_term(*x))
            }
            TmTyApp(f, x) => {
                de::ty_app(self.rename_term(*f), self.rename_type(x))
            }
        }
    }

    fn rename_type(&'a self, t: InputType) -> Type {
        use de::ty;
        use Either::Type;
        use InputType::*;
        match t {
            TyUnit => ty::unit(),
            TyHole => ty::hole(),
            TyVar(x) => ty::var(self.find(Type, x)),
            TyArrow(from, to) => {
                ty::arr(self.rename_type(*from), self.rename_type(*to))
            }
            TyForall(param, body) => ty::forall(
                param.clone(),
                self.with(Type, param).rename_type(*body),
            ),
        }
    }

    fn find(&'a self, kind: Either, name: String) -> Var {
        use Var::*;
        match self.0 {
            Some((_, k, ref n)) if k == kind && *n == name => Bound(0, name),
            Some((prev, k, _)) if k == kind => match prev.find(kind, name) {
                Bound(i, s) => Bound(i + 1, s),
                free => free,
            },
            Some((prev, _, _)) => prev.find(kind, name),
            None => Free(name),
        }
    }

    fn with(&'a self, kind: Either, name: String) -> Self {
        Self(Some((self, kind, name)))
    }
}

pub mod de {
    use super::{Term, TermData::*, Type, Var};

    pub fn unit() -> Term {
        TmUnit.into()
    }

    pub fn abs(
        param: impl Into<String>,
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

    pub fn ty_abs(param: impl Into<String>, body: impl Into<Term>) -> Term {
        TmTyAbs(param.into(), body.into()).into()
    }

    pub fn ty_app(f: impl Into<Term>, ty: impl Into<Type>) -> Term {
        TmTyApp(f.into(), ty.into()).into()
    }

    pub mod ty {
        use crate::bruijn::{Type, TypeData::*, Var};

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

        pub fn forall(param: impl Into<String>, of: impl Into<Type>) -> Type {
            TyForall(param.into(), of.into()).into()
        }
    }
}

impl From<String> for Var {
    fn from(name: String) -> Self {
        Self::Free(name)
    }
}

impl<S> From<(usize, S)> for Var
where
    S: Into<String>,
{
    fn from((depth, name): (usize, S)) -> Self {
        Self::Bound(depth, name.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bruijn::{
            de::{self, ty},
            de_bruijn, Term,
        },
        parser::parse,
    };

    fn de_parsed(input: &str) -> Term {
        de_bruijn(parse(input).unwrap())
    }

    #[test]
    fn triv() {
        assert_eq!(
            de_parsed(r"\x:().x"),
            de::abs("x", ty::unit(), de::var((0, "x")))
        );
    }

    #[test]
    fn breaks_rosser() {
        assert_eq!(
            de_parsed(r"\y:(). (\x:(). \y:(). x) y"),
            de::abs(
                "y",
                ty::unit(),
                de::app(
                    de::abs(
                        "x",
                        ty::unit(),
                        de::abs("y", ty::unit(), de::var((1, "x")))
                    ),
                    de::var((0, "y"))
                )
            )
        );
    }
}
