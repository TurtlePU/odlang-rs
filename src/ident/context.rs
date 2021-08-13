use std::collections::HashSet;

use crate::{
    atoms::*,
    multi_result::MultiResult,
    parser::{inp::*, ty::*},
    syntax::{de, ty, Term, Type},
};

use super::stack::Stack;

#[derive(Default)]
pub struct Context {
    pub names: Names,
    pub alpha: AlphaGen,
    pub stack: Stack,
}

impl Context {
    pub fn rename_term(&mut self, term: InputTerm) -> CtxResult<Term> {
        match term {
            TmUnit => de::unit().into(),
            TmVar(x) => self.rename_var(x, de::var, de::error),
            TmApp(f, x) => (self.rename_term(*f) + self.rename_term(*x))
                .map(|(f, x)| de::app(f, x)),
            TmAbs(x, t, y) => self.rename_arrow(x, |sel, v| {
                (sel.rename_type(t) + sel.rename_term(*y))
                    .map(|(t, y)| de::abs(v, t, y))
            }),
            TmTyAbs(t, x) => self.rename_arrow(t, |sel, v| {
                sel.rename_term(*x).map(|x| de::ty_abs(v, x))
            }),
            TmTyApp(f, x) => (self.rename_term(*f) + self.rename_type(x))
                .map(|(f, x)| de::ty_app(f, x)),
        }
    }

    fn rename_type(&mut self, t: InputType) -> CtxResult<Type> {
        match t {
            TyUnit => ty::unit().into(),
            TyHole => ty::hole(self.alpha.next()).into(),
            TyVar(x) => self.rename_var(x, ty::var, ty::error),
            TyArrow(from, to) => (self.rename_type(*from)
                + self.rename_type(*to))
            .map(|(from, to)| ty::arr(from, to)),
            TyForall(param, body) => self.rename_arrow(param, |sel, v| {
                sel.rename_type(*body).map(|body| ty::forall(v, body))
            }),
        }
    }

    fn rename_var<T>(
        &mut self,
        name: String,
        then: impl FnOnce(Var) -> T,
        err: impl FnOnce(String) -> T,
    ) -> CtxResult<T> {
        match self.stack.map(&name) {
            Some(v) => then(v).into(),
            None => CtxResult::new(err(name.clone()), name),
        }
    }

    fn rename_arrow<T>(
        &mut self,
        name: String,
        then: impl FnOnce(&mut Self, Var) -> T,
    ) -> T {
        let v = self.names.push(name.clone());
        self.stack.push(name.clone(), v);
        let result = then(self, v);
        assert_eq!(self.stack.pop(&name), Some(v));
        result
    }
}

type CtxResult<T> = MultiResult<T, HashSet<String>>;

impl<T> CtxResult<T> {
    fn new(result: T, error: String) -> Self {
        let mut errors = HashSet::default();
        errors.insert(error);
        Self { result, errors }
    }
}
