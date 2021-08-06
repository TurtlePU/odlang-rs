use std::{collections::{HashMap, HashSet, VecDeque}, rc::Rc};

use crate::parser::{
    InputTerm::{self, *},
    InputType::{self, *},
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeData {
    TyUnit,
    TyHole,
    TyVar(Var),
    TyArrow(Type, Type),
    TyForall(Var, Type),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Var(pub usize);

#[derive(Default)]
pub struct Context {
    index: Vec<String>,
    storage: HashMap<String, VecDeque<Var>>,
    unbound: HashSet<String>,
}

impl Context {
    pub fn rename_term(&mut self, term: InputTerm) -> Term {
        match term {
            TmUnit => de::unit(),
            TmVar(x) => de::var(self.find(x)),
            TmApp(f, x) => de::app(self.rename_term(*f), self.rename_term(*x)),
            TmAbs(x, t, y) => {
                let result = de::abs(
                    self.push(x.clone()),
                    self.rename_type(t),
                    self.rename_term(*y),
                );
                self.pop(x);
                result
            }
            TmTyAbs(t, x) => {
                let result =
                    de::ty_abs(self.push(t.clone()), self.rename_term(*x));
                self.pop(t);
                result
            }
            TmTyApp(f, x) => {
                de::ty_app(self.rename_term(*f), self.rename_type(x))
            }
        }
    }

    pub fn rename_type(&mut self, t: InputType) -> Type {
        use de::ty;
        match t {
            TyUnit => ty::unit(),
            TyHole => ty::hole(),
            TyVar(x) => ty::var(self.find(x)),
            TyArrow(from, to) => {
                ty::arr(self.rename_type(*from), self.rename_type(*to))
            }
            TyForall(param, body) => {
                let result = ty::forall(
                    self.push(param.clone()),
                    self.rename_type(*body),
                );
                self.pop(param);
                result
            }
        }
    }

    pub fn name(&self, var: Var) -> String {
        self.index[var.0].clone()
    }

    fn find(&mut self, name: String) -> Var {
        let deque = self.storage.entry(name.clone()).or_default();
        if let Some(var) = deque.back() {
            return *var;
        }
        self.unbound.insert(name.clone());
        let result = intern(&mut self.index, name);
        deque.push_front(result);
        result
    }

    fn push(&mut self, name: String) -> Var {
        let result = intern(&mut self.index, name.clone());
        self.storage.entry(name).or_default().push_back(result);
        result
    }

    fn pop(&mut self, name: String) {
        self.storage.entry(name).or_default().pop_back().unwrap();
    }
}

fn intern(index: &mut Vec<String>, name: String) -> Var {
    index.push(name);
    Var(index.len() - 1)
}

pub mod de {
    use super::{Term, TermData::*, Type, Var};

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

    pub mod ty {
        use crate::intern::{Type, TypeData::*, Var};

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
    }
}

impl From<usize> for Var {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        intern::{
            de::{self, ty},
            Context, Term,
        },
        parser::parse,
    };

    fn parsed(input: &str) -> Term {
        Context::default().rename_term(parse(input).unwrap())
    }

    #[test]
    fn triv() {
        assert_eq!(parsed(r"\x:().x"), de::abs(0, ty::unit(), de::var(0)));
    }

    #[test]
    fn breaks_rosser() {
        assert_eq!(
            parsed(r"\y:(). (\x:(). \y:(). x) y"),
            de::abs(
                0,
                ty::unit(),
                de::app(
                    de::abs(1, ty::unit(), de::abs(2, ty::unit(), de::var(1))),
                    de::var(0)
                )
            )
        );
    }
}
