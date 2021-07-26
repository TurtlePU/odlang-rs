use crate::{
    bruijn::{de, DeBruijnTerm, Type},
    typeck::{self, shift_type},
    var::Var,
};

pub fn eval(term: DeBruijnTerm) -> DeBruijnTerm {
    use DeBruijnTerm::*;
    match term {
        TmApp(f, x) => match (eval(*f), eval(*x)) {
            (TmAbs(_, y), x) => eval(unshift(subst(shift(x, 1), *y))),
            (f, x) => de::app(f, x),
        },
        TmTyApp(f, t) => match eval(*f) {
            TmTyAbs(y) => {
                eval(unshift_type(subst_type(shift_type(t, 0), *y, 0), 0))
            }
            term => term,
        },
        term => term,
    }
}

fn subst(with: DeBruijnTerm, inside: DeBruijnTerm) -> DeBruijnTerm {
    Substitutor(with).sub(inside, 0)
}

fn shift(term: DeBruijnTerm, inc: usize) -> DeBruijnTerm {
    Shifter(|x| x + inc).shift(term, 0)
}

fn unshift(term: DeBruijnTerm) -> DeBruijnTerm {
    Shifter(|x| x - 1).shift(term, 0)
}

fn subst_type(with: Type, term: DeBruijnTerm, depth: usize) -> DeBruijnTerm {
    use DeBruijnTerm::*;
    match term {
        TmUnit => de::unit(),
        TmVar(k) => de::var(k),
        TmAbs(ty, y) => de::abs(typeck::subst_type(ty, with, depth), *y),
        TmApp(f, x) => de::app(
            subst_type(with.clone(), *f, depth),
            subst_type(with, *x, depth),
        ),
        TmTyAbs(body) => de::ty_abs(subst_type(with, *body, depth + 1)),
        TmTyApp(f, x) => de::ty_app(
            subst_type(with.clone(), *f, depth),
            typeck::subst_type(x, with, depth),
        ),
    }
}

fn unshift_type(term: DeBruijnTerm, thr: usize) -> DeBruijnTerm {
    use DeBruijnTerm::*;
    match term {
        TmUnit => de::unit(),
        TmVar(k) => de::var(k),
        TmAbs(ty, y) => de::abs(typeck::unshift_type(ty, thr), *y),
        TmApp(f, x) => de::app(unshift_type(*f, thr), unshift_type(*x, thr)),
        TmTyAbs(body) => de::ty_abs(unshift_type(*body, thr + 1)),
        TmTyApp(f, x) => {
            de::ty_app(unshift_type(*f, thr), typeck::unshift_type(x, thr))
        }
    }
}

struct Substitutor(DeBruijnTerm);

impl Substitutor {
    fn sub(&self, inside: DeBruijnTerm, depth: usize) -> DeBruijnTerm {
        use DeBruijnTerm::*;
        use Var::Bound;
        match inside {
            TmVar(Bound(i)) if i == depth => shift(self.0.clone(), depth),
            TmAbs(ty, y) => de::abs(ty, self.sub(*y, depth + 1)),
            TmApp(f, x) => de::app(self.sub(*f, depth), self.sub(*x, depth)),
            inside => inside,
        }
    }
}

struct Shifter<F>(F);

impl<F> Shifter<F>
where
    F: Fn(usize) -> usize,
{
    fn shift(&self, term: DeBruijnTerm, thr: usize) -> DeBruijnTerm {
        use DeBruijnTerm::*;
        use Var::Bound;
        match term {
            TmVar(Bound(x)) if x >= thr => de::var((self.0)(x)),
            TmAbs(ty, y) => de::abs(ty, self.shift(*y, thr + 1)),
            TmApp(f, x) => de::app(self.shift(*f, thr), self.shift(*x, thr)),
            term => term,
        }
    }
}
