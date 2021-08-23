use std::{collections::HashSet, error::Error, fmt::Display};

use crate::{
    input::*,
    prelude::*,
    syntax::{de, ty, Term, Type},
};

pub type IdResult = Result<(Term, Names), Unbound>;

pub fn identify(term: InputTerm) -> IdResult {
    let MultiResult {
        result,
        state,
        errors,
    } = rename_term(term)(&Stack::default(), Names::default());
    if errors.0.is_empty() {
        Ok((result, state))
    } else {
        Err(errors)
    }
}

type CtxResult<T> = MultiResult<T, Names, Unbound>;

#[derive(Default)]
struct Stack<'a>(Option<(&'a Self, String, Var)>);

fn rename_term(
    term: InputTerm,
) -> impl FnOnce(&Stack, Names) -> CtxResult<Term> {
    move |stack, context| match term {
        TmUnit(_) => CtxResult::ok(de::unit(), context),
        TmVar(_, name) => find_var(name)(stack, context),
        TmAbs(_, name, ty, term) => new_var(name, |var| {
            fmap2(
                pipe2(rename_type(ty), rename_term(*term)),
                move |(ty, term)| de::abs(var, ty, term),
            )
        })(stack, context),
        TmApp(_, f, x) => {
            fmap2(pipe2(rename_term(*f), rename_term(*x)), |(f, x)| {
                de::app(f, x)
            })(stack, context)
        }
        TmTyAbs(_, name, term) => new_var(name, |var| {
            fmap2(rename_term(*term), move |term| de::ty_abs(var, term))
        })(stack, context),
        TmTyApp(_, f, x) => {
            fmap2(pipe2(rename_term(*f), rename_type(x)), |(f, x)| {
                de::ty_app(f, x)
            })(stack, context)
        }
        TmError => unreachable!(),
    }
}

fn rename_type(
    input_type: InputType,
) -> impl FnOnce(&Stack, Names) -> CtxResult<Type> {
    move |stack, names| match input_type {
        TyUnit(_) => CtxResult::ok(ty::unit(), names),
        TyHole(_) => CtxResult::ok(ty::hole(), names),
        TyVar(_, name) => find_var(name)(stack, names),
        TyArrow(_, from, to) => {
            fmap2(pipe2(rename_type(*from), rename_type(*to)), |(from, to)| {
                ty::arr(from, to)
            })(stack, names)
        }
        TyForall(_, name, ty) => new_var(name, |v| {
            fmap2(rename_type(*ty), move |ty| ty::forall(v, ty))
        })(stack, names),
        TyError => unreachable!(),
    }
}

fn new_var<F, T>(
    name: String,
    then: impl FnOnce(Var) -> F,
) -> impl FnOnce(&Stack, Names) -> T
where
    F: FnOnce(&Stack, Names) -> T,
{
    move |stack, mut names| {
        let var = names.push(name.clone());
        let stack = stack.push(name, var);
        then(var)(&stack, names)
    }
}

fn find_var<T>(name: String) -> impl FnOnce(&Stack, Names) -> CtxResult<T>
where
    T: From<Var> + ErrValue,
{
    move |stack, state| match stack.map(name) {
        Ok(var) => CtxResult::ok(var.into(), state),
        Err(error) => CtxResult::err(state, error),
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
}

fn pipe2<T, U>(
    first: impl FnOnce(&Stack, Names) -> CtxResult<T>,
    second: impl FnOnce(&Stack, Names) -> CtxResult<U>,
) -> impl FnOnce(&Stack, Names) -> CtxResult<(T, U)> {
    |stack, names| first(stack, names) >> |names| second(stack, names)
}

fn fmap2<T, U>(
    gen: impl FnOnce(&Stack, Names) -> CtxResult<T>,
    map: impl FnOnce(T) -> U,
) -> impl FnOnce(&Stack, Names) -> CtxResult<U> {
    move |stack, names| gen(stack, names).map(map)
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
