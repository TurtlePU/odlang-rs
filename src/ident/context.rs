use crate::{
    atoms::*,
    parser::{inp::*, ty::*},
    syntax::{de, ty, Term, Type},
};

use super::{stack::Stack, unbound::Unbound};

#[derive(Default)]
pub struct Context {
    names: Names,
    alpha: AlphaGen,
    stack: Stack,
    unbound: Unbound,
}

impl Context {
    pub fn rename_term(&mut self, term: InputTerm) -> Term {
        match term {
            TmUnit => de::unit(),
            TmVar(x) => self.rename_var(x, de::var, de::error),
            TmApp(f, x) => de::app(self.rename_term(*f), self.rename_term(*x)),
            TmAbs(x, t, y) => self.rename_arrow(x, |sel, v| {
                de::abs(v, sel.rename_type(t), sel.rename_term(*y))
            }),
            TmTyAbs(t, x) => self
                .rename_arrow(t, |sel, v| de::ty_abs(v, sel.rename_term(*x))),
            TmTyApp(f, x) => {
                de::ty_app(self.rename_term(*f), self.rename_type(x))
            }
        }
    }

    fn rename_type(&mut self, t: InputType) -> Type {
        match t {
            TyUnit => ty::unit(),
            TyHole => ty::hole(self.alpha.next()),
            TyVar(x) => self.rename_var(x, ty::var, ty::error),
            TyArrow(from, to) => {
                ty::arr(self.rename_type(*from), self.rename_type(*to))
            }
            TyForall(param, body) => self.rename_arrow(param, |sel, v| {
                ty::forall(v, sel.rename_type(*body))
            }),
        }
    }

    fn rename_var<T>(
        &mut self,
        name: String,
        then: impl FnOnce(Var) -> T,
        err: impl FnOnce(String) -> T,
    ) -> T {
        match self.stack.map(&name) {
            Some(v) => then(v),
            None => {
                self.unbound.report(name.clone());
                err(name)
            }
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

    pub fn terminate(self) -> Result<(Names, AlphaGen), Unbound> {
        let Context {
            names,
            alpha,
            stack,
            unbound,
        } = self;
        assert!(stack.is_empty());
        if unbound.is_empty() {
            Ok((names, alpha))
        } else {
            Err(unbound)
        }
    }
}
