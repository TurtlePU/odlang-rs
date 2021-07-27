use crate::parser::{InputTerm, InputType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    TmUnit,
    TmVar(Var),
    TmAbs(String, Type, Box<Term>),
    TmApp(Box<Term>, Box<Term>),
    TmTyAbs(String, Box<Term>),
    TmTyApp(Box<Term>, Type),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    TyUnit,
    TyHole,
    TyVar(Var),
    TyArrow(Box<Type>, Box<Type>),
    TyForall(String, Box<Type>),
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
    use super::{
        Term::{self, *},
        Type, Var,
    };

    pub fn unit() -> Term {
        TmUnit
    }

    pub fn abs(param: impl Into<String>, r#type: Type, body: Term) -> Term {
        TmAbs(param.into(), r#type, Box::new(body))
    }

    pub fn app(f: Term, x: Term) -> Term {
        TmApp(Box::new(f), Box::new(x))
    }

    pub fn var(key: impl Into<Var>) -> Term {
        TmVar(key.into())
    }

    pub fn ty_abs(param: impl Into<String>, body: Term) -> Term {
        TmTyAbs(param.into(), Box::new(body))
    }

    pub fn ty_app(f: Term, ty: Type) -> Term {
        TmTyApp(Box::new(f), ty)
    }

    pub mod ty {
        use super::{Type, Var};

        pub fn unit() -> Type {
            Type::TyUnit
        }

        pub fn hole() -> Type {
            Type::TyHole
        }

        pub fn var(key: impl Into<Var>) -> Type {
            Type::TyVar(key.into())
        }

        pub fn arr(from: Type, to: Type) -> Type {
            Type::TyArrow(Box::new(from), Box::new(to))
        }

        pub fn forall(param: impl Into<String>, of: Type) -> Type {
            Type::TyForall(param.into(), Box::new(of))
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
