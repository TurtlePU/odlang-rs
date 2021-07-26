use crate::bruijn::{de, DeBruijnTerm, Var};

pub fn eval(term: DeBruijnTerm) -> DeBruijnTerm {
    use DeBruijnTerm::{TmApp, TmAbs};
    match term {
        TmApp(f, x) => match (eval(*f), eval(*x)) {
            (TmAbs(_, y), x) => eval(unshift(subst(shift(x, 1), *y))),
            (f, x) => de::app(f, x)
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
