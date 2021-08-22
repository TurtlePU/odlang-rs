use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::{atoms::*, multi_result::*, syntax::*};

pub fn typeck(gen: AlphaGen, term: Term) -> Result<Type, TypeckErrors> {
    let MultiResult {
        result,
        state: _,
        errors,
    } = typeck_term(term)((HashMap::default(), gen));
    if errors.is_empty() {
        Ok(result)
    } else {
        Err(errors)
    }
}

pub fn subst_type(body: Type, with: Type, what: Var) -> Type {
    match (*body).clone() {
        TyUnit => body,
        TyAlpha(_) => body,
        TyVar(var) if var == what => with,
        TyVar(_) => body,
        TyArrow(from, to) => ty::arr(
            subst_type(from, with.clone(), what),
            subst_type(to, with, what),
        ),
        TyForall(n, x) => ty::forall(n, subst_type(x, with, what)),
        TyError => unreachable!(),
    }
}

pub type TypeckErrors = VecDeque<TypeckError>;

impl Named for TypeckErrors {
    fn pprint(&self, names: &Names) -> String {
        self.iter().map(|err| err.pprint(names)).join("\n")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeckError {
    NotAFunction(Type),
    NotAForall(Type),
    NotEqual(Type, Type),
}

use TypeckError::*;

impl Named for TypeckError {
    fn pprint(&self, names: &Names) -> String {
        match self {
            NotEqual(a, b) => format!(
                "Types should be equal: '{}', '{}'",
                a.pprint(names),
                b.pprint(names)
            ),
            NotAFunction(f) => {
                format!("Must be a function: '{}'", f.pprint(names))
            }
            NotAForall(f) => {
                format!("Must be a forall: '{}'", f.pprint(names))
            }
        }
    }
}

type Typeck = (HashMap<Var, Type>, AlphaGen);

type TypeckResult = MultiResult<Type, Typeck, VecDeque<TypeckError>>;

fn typeck_term(term: Term) -> impl FnOnce(Typeck) -> TypeckResult {
    move |mut state| match (*term).clone() {
        TmUnit => TypeckResult::ok(ty::unit(), state),
        TmVar(v) => TypeckResult::ok(get_or_alpha(&mut state, v), state),
        TmAbs(v, t, y) => {
            let tt = t.clone();
            fmap(typeck_term(y), move |y| ty::arr(tt, y))(insert(state, v, t))
        }
        TmApp(f, x) => fthen(pipe(typeck_term(f), typeck_term(x)), |(f, x)| {
            assert_app(f, x)
        })(state),
        TmTyAbs(n, x) => fmap(typeck_term(x), |x| ty::forall(n, x))(state),
        TmTyApp(f, t) => fthen(typeck_term(f), |f| assert_ty_app(f, t))(state),
        TmError => unreachable!(),
    }
}

fn get_or_alpha(state: &mut Typeck, v: Var) -> Type {
    state.0.get(&v).cloned().unwrap_or_else(|| next_hole(state))
}

fn insert(mut state: Typeck, v: Var, t: Type) -> Typeck {
    state.0.insert(v, t);
    state
}

fn assert_app(fun: Type, arg: Type) -> impl FnOnce(Typeck) -> TypeckResult {
    move |state| match (*fun).clone() {
        TyArrow(from, to) if from == arg => TypeckResult::ok(to, state),
        TyArrow(from, to) => TypeckResult::new(to, state, NotEqual(from, arg)),
        _ => TypeckResult::err(state, NotAFunction(fun)),
    }
}

fn assert_ty_app(fun: Type, arg: Type) -> impl FnOnce(Typeck) -> TypeckResult {
    move |state| match (*fun).clone() {
        TyForall(var, inner) => {
            TypeckResult::ok(subst_type(inner, arg, var), state)
        }
        _ => TypeckResult::err(state, NotAForall(fun)),
    }
}

fn next_hole(state: &mut Typeck) -> Type {
    ty::hole(state.1.next())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typeck(term: Term) -> Result<Type, VecDeque<TypeckError>> {
        super::typeck(AlphaGen::default(), term)
    }

    #[test]
    fn simple_typeck() {
        assert_eq!(
            typeck(de::abs(0, ty::unit(), de::var(0))),
            Ok(ty::arr(ty::unit(), ty::unit()))
        );
    }
}
