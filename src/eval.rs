use crate::{
    bruijn::{de, Term, Type, Var},
    typeck,
};

pub fn eval(term: Term) -> Term {
    use Term::*;
    match term {
        TmApp(f, x) => match (eval(*f), eval(*x)) {
            (TmAbs(_, _, y), x) => eval(unshift(subst(shift(x), *y))),
            (f, x) => de::app(f, x),
        },
        TmTyApp(f, t) => match eval(*f) {
            TmTyAbs(_, y) => eval(unshift_type(subst_type(shift_type(t), *y))),
            term => de::ty_app(term, t),
        },
        term => term,
    }
}

fn shift(term: Term) -> Term {
    do_shift(&|x| x + 1, term, 0)
}

fn unshift(term: Term) -> Term {
    do_shift(&|x| x - 1, term, 0)
}

fn shift_type(ty: Type) -> Type {
    typeck::shift_type(ty, 0)
}

fn subst_type(with: Type, term: Term) -> Term {
    fn do_subst_type(with: Type, term: Term, depth: usize) -> Term {
        use Term::*;
        match term {
            TmUnit => de::unit(),
            TmVar(k) => de::var(k),
            TmAbs(n, ty, y) => {
                de::abs(n, typeck::subst_type(ty, with, depth), *y)
            }
            TmApp(f, x) => de::app(
                do_subst_type(with.clone(), *f, depth),
                do_subst_type(with, *x, depth),
            ),
            TmTyAbs(n, body) => de::ty_abs(n, do_subst_type(with, *body, depth + 1)),
            TmTyApp(f, x) => de::ty_app(
                do_subst_type(with.clone(), *f, depth),
                typeck::subst_type(x, with, depth),
            ),
        }
    }
    do_subst_type(with, term, 0)
}

fn unshift_type(term: Term) -> Term {
    fn do_unshift_type(term: Term, thr: usize) -> Term {
        use Term::*;
        match term {
            TmUnit => de::unit(),
            TmVar(k) => de::var(k),
            TmAbs(n, ty, y) => de::abs(n, typeck::unshift_type(ty, thr), *y),
            TmApp(f, x) => {
                de::app(do_unshift_type(*f, thr), do_unshift_type(*x, thr))
            }
            TmTyAbs(n, body) => de::ty_abs(n, do_unshift_type(*body, thr + 1)),
            TmTyApp(f, x) => de::ty_app(
                do_unshift_type(*f, thr),
                typeck::unshift_type(x, thr),
            ),
        }
    }
    do_unshift_type(term, 0)
}

fn subst(term: Term, inside: Term) -> Term {
    fn do_subst(term: Term, inside: Term, depth: usize) -> Term {
        use Term::*;
        use Var::Bound;
        match inside {
            TmUnit => de::unit(),
            TmVar(Bound(i, _)) if i == depth => {
                do_shift(&|i| i + depth, term, 0)
            }
            TmVar(other) => de::var(other),
            TmAbs(n, ty, y) => de::abs(n, ty, do_subst(term, *y, depth + 1)),
            TmApp(f, x) => de::app(
                do_subst(term.clone(), *f, depth),
                do_subst(term, *x, depth),
            ),
            TmTyAbs(n, y) => de::ty_abs(n, do_subst(term, *y, depth)),
            TmTyApp(f, t) => de::ty_app(do_subst(term, *f, depth), t),
        }
    }
    do_subst(term, inside, 0)
}

fn do_shift(how: &impl Fn(usize) -> usize, term: Term, thr: usize) -> Term {
    use Term::*;
    use Var::Bound;
    match term {
        TmUnit => de::unit(),
        TmVar(Bound(x, n)) if x >= thr => de::var((how(x), n)),
        TmVar(other) => de::var(other),
        TmAbs(n, ty, y) => de::abs(n, ty, do_shift(how, *y, thr + 1)),
        TmApp(f, x) => de::app(do_shift(how, *f, thr), do_shift(how, *x, thr)),
        TmTyAbs(n, y) => de::ty_abs(n, do_shift(how, *y, thr)),
        TmTyApp(f, t) => de::ty_app(do_shift(how, *f, thr), t),
    }
}
