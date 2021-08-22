use std::{collections::HashSet, error::Error, fmt::Display};

use crate::{
    prelude::*,
    input::*,
    syntax::{de, ty, Term, Type},
};

pub type IdResult = Result<(Term, Names, AlphaGen), Unbound>;

pub fn identify(term: InputTerm) -> IdResult {
    let MultiResult {
        result,
        state: (names, alpha),
        errors,
    } = rename_term(term)(&Stack::default(), Context::default());
    if errors.0.is_empty() {
        Ok((result, names, alpha))
    } else {
        Err(errors)
    }
}

type CtxResult<T> = MultiResult<T, Context, Unbound>;

type Context = (Names, AlphaGen);

#[derive(Default)]
struct Stack<'a>(Option<(&'a Self, String, Var)>);

fn rename_term(
    term: InputTerm,
) -> impl FnOnce(&Stack, Context) -> CtxResult<Term> {
    move |stack, context| match term {
        TmUnit(_) => CtxResult::ok(de::unit(), context),
        TmVar(_, name) => match stack.map(name) {
            Ok(v) => CtxResult::ok(de::var(v), context),
            Err(name) => CtxResult::err(context, name),
        },
        TmAbs(_, name, ty, term) => new_var(stack, name, |v| {
            fmap2(
                pipe2(rename_type(ty), rename_term(*term)),
                move |(ty, term)| de::abs(v, ty, term),
            )
        })(context),
        TmApp(_, f, x) => {
            fmap2(pipe2(rename_term(*f), rename_term(*x)), |(f, x)| {
                de::app(f, x)
            })(stack, context)
        }
        TmTyAbs(_, name, term) => new_var(stack, name, |v| {
            fmap2(rename_term(*term), move |term| de::ty_abs(v, term))
        })(context),
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
) -> impl FnOnce(&Stack, Context) -> CtxResult<Type> {
    move |stack, mut context| match input_type {
        TyUnit(_) => CtxResult::ok(ty::unit(), context),
        TyHole(_) => CtxResult::ok(ty::hole(context.1.next()), context),
        TyVar(_, name) => match stack.map(name) {
            Ok(v) => CtxResult::ok(ty::var(v), context),
            Err(err) => CtxResult::err(context, err),
        },
        TyArrow(_, from, to) => {
            fmap2(pipe2(rename_type(*from), rename_type(*to)), |(from, to)| {
                ty::arr(from, to)
            })(stack, context)
        }
        TyForall(_, name, ty) => new_var(stack, name, |v| {
            fmap2(rename_type(*ty), move |ty| ty::forall(v, ty))
        })(context),
        TyError => unreachable!(),
    }
}

fn new_var<'a, F, T>(
    s: &'a Stack,
    n: String,
    then: impl FnOnce(Var) -> F + 'a,
) -> impl FnOnce(Context) -> T + 'a
where
    F: FnOnce(&Stack, Context) -> T + 'a,
{
    move |mut c| {
        let v = c.0.push(n.clone());
        let stack = s.push(n, v);
        then(v)(&stack, c)
    }
}

impl<'a> Stack<'a> {
    fn push(&'a self, name: String, v: Var) -> Self {
        Self(Some((self, name, v)))
    }

    fn map(&self, name: String) -> Result<Var, String> {
        match self.0 {
            Some((_, ref key, v)) if *key == name => Ok(v),
            Some((prev, _, _)) => prev.map(name),
            None => Err(name),
        }
    }
}

fn pipe2<T, U>(
    first: impl FnOnce(&Stack, Context) -> CtxResult<T>,
    second: impl FnOnce(&Stack, Context) -> CtxResult<U>,
) -> impl FnOnce(&Stack, Context) -> CtxResult<(T, U)> {
    |stack, context| first(stack, context) >> |context| second(stack, context)
}

fn fmap2<T, U>(
    gen: impl FnOnce(&Stack, Context) -> CtxResult<T>,
    map: impl FnOnce(T) -> U,
) -> impl FnOnce(&Stack, Context) -> CtxResult<U> {
    move |stack, context| gen(stack, context).map(map)
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
