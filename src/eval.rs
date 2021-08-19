use crate::{syntax::*, atoms::Var, typeck};

pub fn eval(term: Term) -> Term {
    match (*term).clone() {
        TmApp(f, x) => match ((*eval(f)).clone(), eval(x)) {
            (TmAbs(v, _, y), x) => eval(subst(x, y, v)),
            (f, x) => de::app(f, x),
        },
        TmTyApp(f, t) => match (*eval(f)).clone() {
            TmTyAbs(v, y) => eval(subst_type(t, y, v)),
            term => de::ty_app(term, t),
        },
        _ => term,
    }
}

fn subst_type(with: Type, term: Term, var: Var) -> Term {
    match (*term).clone() {
        TmUnit => term,
        TmVar(_) => term,
        TmAbs(n, ty, y) => de::abs(n, typeck::subst_type(ty, with, var), y),
        TmApp(f, x) => {
            de::app(subst_type(with.clone(), f, var), subst_type(with, x, var))
        }
        TmTyAbs(n, body) => de::ty_abs(n, subst_type(with, body, var)),
        TmTyApp(f, x) => de::ty_app(
            subst_type(with.clone(), f, var),
            typeck::subst_type(x, with, var),
        ),
        TmError => unreachable!()
    }
}

fn subst(with: Term, inside: Term, what: Var) -> Term {
    match (*inside).clone() {
        TmUnit => inside,
        TmVar(var) if var == what => with,
        TmVar(other) => inside,
        TmAbs(n, ty, y) => de::abs(n, ty, subst(with, y, what)),
        TmApp(f, x) => {
            de::app(subst(with.clone(), f, what), subst(with, x, what))
        }
        TmTyAbs(n, y) => de::ty_abs(n, subst(with, y, what)),
        TmTyApp(f, t) => de::ty_app(subst(with, f, what), t),
        TmError => unreachable!()
    }
}
