use crate::intern::{Context, Term, TermData::*, Type, TypeData::*};

pub fn pprint(int: &Context, term: Term) -> String {
    match (*term).clone() {
        TmUnit => "()".into(),
        TmVar(var) => int.name(var),
        TmAbs(n, t, y) => {
            format!(
                "\\{}: {}. {}",
                int.name(n),
                pprint_type(int, t),
                pprint(int, y)
            )
        }
        TmApp(f, x) => match *f {
            TmAbs(_, _, _) => {
                format!("({}) {}", pprint(int, f), pprint(int, x))
            }
            _ => format!("{} {}", pprint(int, f), pprint(int, x)),
        },
        TmTyAbs(n, y) => format!("/\\ {}. {}", int.name(n), pprint(int, y)),
        TmTyApp(f, x) => match *f {
            TmTyAbs(_, _) => {
                format!("({}) [{}]", pprint(int, f), pprint_type(int, x))
            }
            _ => format!("{} [{}]", pprint(int, f), pprint_type(int, x)),
        },
    }
}

fn pprint_type(int: &Context, ty: Type) -> String {
    match (*ty).clone() {
        TyUnit => "()".into(),
        TyHole => "_".into(),
        TyVar(var) => int.name(var),
        TyArrow(f, t) => match *f {
            TyUnit | TyHole | TyVar(_) => {
                format!("{} -> {}", pprint_type(int, f), pprint_type(int, t))
            }
            _ => {
                format!("({}) -> {}", pprint_type(int, f), pprint_type(int, t))
            }
        },
        TyForall(n, y) => {
            format!("/\\ {} => {}", int.name(n), pprint_type(int, y))
        }
    }
}
