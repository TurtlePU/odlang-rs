use crate::bruijn::{Term, Type, Var};

pub fn pprint(term: Term) -> String {
    use Term::*;
    use Var::*;
    match term {
        TmUnit => "()".into(),
        TmVar(Bound(_, s) | Free(s)) => s,
        TmAbs(n, t, y) => {
            format!("\\{}: {}. {}", n, pprint_type(t), pprint(*y))
        }
        TmApp(f, x) => match *f {
            TmAbs(_, _, _) => format!("({}) {}", pprint(*f), pprint(*x)),
            _ => format!("{} {}", pprint(*f), pprint(*x)),
        },
        TmTyAbs(n, y) => format!("/\\ {}. {}", n, pprint(*y)),
        TmTyApp(f, x) => match *f {
            TmTyAbs(_, _) => format!("({}) [{}]", pprint(*f), pprint_type(x)),
            _ => format!("{} [{}]", pprint(*f), pprint_type(x)),
        },
    }
}

fn pprint_type(ty: Type) -> String {
    use Type::*;
    use Var::*;
    match ty {
        TyUnit => "()".into(),
        TyHole => "_".into(),
        TyVar(Bound(_, s) | Free(s)) => s,
        TyArrow(f, t) => match *f {
            TyUnit | TyHole | TyVar(_) => {
                format!("{} -> {}", pprint_type(*f), pprint_type(*t))
            }
            _ => format!("({}) -> {}", pprint_type(*f), pprint_type(*t)),
        },
        TyForall(n, y) => format!("/\\ {} => {}", n, pprint_type(*y)),
    }
}
