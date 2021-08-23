use std::{collections::HashSet, error::Error, fmt::Display};

use crate::{
    input::*,
    prelude::*,
    syntax::{de, ty, Term, Type},
};

pub type IdResult = Result<(Term, Names), Unbound>;

pub fn identify(term: InputTerm) -> IdResult {
    let mut names = Names::default();
    let MultiResult { result, collect } =
        names.rename_term(&Stack::default(), term);
    if collect.0.is_empty() {
        Ok((result, names))
    } else {
        Err(collect)
    }
}

type CtxResult<T> = MultiResult<T, Unbound>;

#[derive(Default)]
struct Stack<'a>(Option<(&'a Self, String, Var)>);

impl Names {
    fn rename_term(
        &mut self,
        stack: &Stack,
        term: InputTerm,
    ) -> CtxResult<Term> {
        match term {
            TmUnit(_) => de::unit().into(),
            TmVar(_, name) => stack.find_var(name),
            TmAbs(_, name, ty, term) => {
                let (var, ref stack) = self.new_var(stack, name);
                (self.rename_type(stack, ty) + self.rename_term(stack, *term))
                    .map(|(ty, term)| de::abs(var, ty, term))
            }
            TmApp(_, f, x) => (self.rename_term(stack, *f)
                + self.rename_term(stack, *x))
            .map(|(f, x)| de::app(f, x)),
            TmTyAbs(_, name, term) => {
                let (var, ref stack) = self.new_var(stack, name);
                self.rename_term(stack, *term)
                    .map(move |term| de::ty_abs(var, term))
            }
            TmTyApp(_, f, x) => (self.rename_term(stack, *f)
                + self.rename_type(stack, x))
            .map(|(f, x)| de::ty_app(f, x)),
            TmError => unreachable!(),
        }
    }

    fn rename_type(
        &mut self,
        stack: &Stack,
        input_type: InputType,
    ) -> CtxResult<Type> {
        match input_type {
            TyUnit(_) => ty::unit().into(),
            TyHole(_) => ty::hole().into(),
            TyVar(_, name) => stack.find_var(name),
            TyArrow(_, from, to) => (self.rename_type(stack, *from)
                + self.rename_type(stack, *to))
            .map(|(from, to)| ty::arr(from, to)),
            TyForall(_, name, ty) => {
                let (var, ref stack) = self.new_var(stack, name);
                self.rename_type(stack, *ty)
                    .map(move |ty| ty::forall(var, ty))
            }
            TyError => unreachable!(),
        }
    }

    fn new_var<'a>(
        &mut self,
        stack: &'a Stack,
        name: String,
    ) -> (Var, Stack<'a>) {
        let var = self.push(name.clone());
        let stack = stack.push(name, var);
        (var, stack)
    }
}

impl<'a> Stack<'a> {
    fn push(&'a self, name: String, var: Var) -> Self {
        Self(Some((self, name, var)))
    }

    fn map(&self, name: String) -> Result<Var, String> {
        match self.0 {
            Some((_, ref key, var)) if *key == name => Ok(var),
            Some((prev, _, _)) => prev.map(name),
            None => Err(name),
        }
    }

    fn find_var<T>(&self, name: String) -> CtxResult<T>
    where
        T: From<Var> + Default,
    {
        match self.map(name) {
            Ok(var) => T::from(var).into(),
            Err(error) => CtxResult::item(error),
        }
    }
}

#[derive(Default, Debug)]
pub struct Unbound(HashSet<String>);

impl Error for Unbound {}

impl Display for Unbound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for name in &self.0 {
            writeln!(f, "Unbound name: {}", name)?;
        }
        Ok(())
    }
}

impl Semigroup for Unbound {
    fn app(self, other: Self) -> Self {
        Self(self.0.app(other.0))
    }
}

impl<T> Singleton<T> for Unbound
where
    HashSet<String>: Singleton<T>,
{
    fn single(elem: T) -> Self {
        Self(HashSet::single(elem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parser::parse,
        syntax::{de, ty},
    };

    fn parsed(input: &str) -> IdResult {
        identify(parse(input).unwrap())
    }

    #[test]
    fn triv() {
        assert_eq!(
            parsed(r"\x:().x").unwrap().0,
            de::abs(0, ty::unit(), de::var(0))
        );
    }

    #[test]
    fn breaks_rosser() {
        assert_eq!(
            parsed(r"\y:(). (\x:(). \y:(). x) y").unwrap().0,
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
