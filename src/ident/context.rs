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
}

impl Context {
    pub fn rename_term(
        &mut self,
        stack: &Stack,
        term: InputTerm,
    ) -> CtxResult<Term> {
        match term {
            TmUnit => de::unit().into(),
            TmVar(x) => rename_var(stack, x, de::var, de::error),
            TmApp(f, x) => (self.rename_term(stack, *f)
                + self.rename_term(stack, *x))
            .map(|(f, x)| de::app(f, x)),
            TmAbs(x, t, y) => self.rename_arrow(stack, x, |sel, stack, v| {
                (sel.rename_type(stack, t) + sel.rename_term(stack, *y))
                    .map(|(t, y)| de::abs(v, t, y))
            }),
            TmTyAbs(t, x) => self.rename_arrow(stack, t, |sel, stack, v| {
                sel.rename_term(stack, *x).map(|x| de::ty_abs(v, x))
            }),
            TmTyApp(f, x) => (self.rename_term(stack, *f)
                + self.rename_type(stack, x))
            .map(|(f, x)| de::ty_app(f, x)),
            TmError => unreachable!(),
        }
    }

    fn rename_type(&mut self, stack: &Stack, t: InputType) -> CtxResult<Type> {
        match t {
            TyUnit => ty::unit().into(),
            TyHole => ty::hole(self.alpha.next()).into(),
            TyVar(x) => rename_var(stack, x, ty::var, ty::error),
            TyArrow(from, to) => (self.rename_type(stack, *from)
                + self.rename_type(stack, *to))
            .map(|(from, to)| ty::arr(from, to)),
            TyForall(param, body) => {
                self.rename_arrow(stack, param, |sel, stack, v| {
                    sel.rename_type(stack, *body)
                        .map(|body| ty::forall(v, body))
                })
            },
            TyError => unreachable!(),
        }
    }

    fn rename_arrow<T>(
        &mut self,
        stack: &Stack,
        name: String,
        then: impl FnOnce(&mut Self, &Stack, Var) -> T,
    ) -> T {
        let v = self.names.push(name.clone());
        let result = then(self, &stack.push(name, v), v);
        result
    }
}

fn rename_var<T>(
    stack: &Stack,
    name: String,
    then: impl FnOnce(Var) -> T,
    err: impl FnOnce() -> T,
) -> CtxResult<T> {
    match stack.map(name) {
        Ok(v) => then(v).into(),
        Err(name) => CtxResult::err(err(), name),
    }
}

type CtxResult<T> = MultiResult<T, (), HashSet<String>>;
