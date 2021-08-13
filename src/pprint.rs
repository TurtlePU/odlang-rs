use itertools::Itertools;

use crate::{
    ident::{Term, TermData::*, Type, TypeData::*},
    names::Names,
    typeck::TypeckError,
};

pub fn pprint(names: &Names, term: Term) -> String {
    match (*term).clone() {
        TmUnit => "()".into(),
        TmVar(var) => names[var].clone(),
        TmAbs(n, t, y) => {
            format!(
                "\\{}: {}. {}",
                names[n],
                pprint_type(names, t),
                pprint(names, y)
            )
        }
        TmApp(f, x) => match *f {
            TmAbs(_, _, _) => {
                format!("({}) {}", pprint(names, f), pprint(names, x))
            }
            _ => format!("{} {}", pprint(names, f), pprint(names, x)),
        },
        TmTyAbs(n, y) => format!("/\\ {}. {}", names[n], pprint(names, y)),
        TmTyApp(f, x) => match *f {
            TmTyAbs(_, _) => {
                format!("({}) [{}]", pprint(names, f), pprint_type(names, x))
            }
            _ => format!("{} [{}]", pprint(names, f), pprint_type(names, x)),
        },
        TmError(err) => err,
    }
}

pub fn pprint_errors(names: &Names, errs: Vec<TypeckError>) -> String {
    errs.into_iter()
        .map(|err| pprint_error(names, err))
        .join("\n")
}

fn pprint_error(names: &Names, err: TypeckError) -> String {
    use TypeckError::*;
    match err {
        NotEqual(a, b) => format!(
            "Types should be equal: '{}', '{}'",
            pprint_type(names, a),
            pprint_type(names, b)
        ),
        NotAFunction(f) => {
            format!("Must be a function: '{}'", pprint_type(names, f))
        }
        NotAForall(f) => {
            format!("Must be a forall: '{}'", pprint_type(names, f))
        }
    }
}

fn pprint_type(names: &Names, ty: Type) -> String {
    match (*ty).clone() {
        TyUnit => "()".into(),
        TyAlpha(alp) => format!("{}", alp),
        TyVar(var) => names[var].clone(),
        TyArrow(f, t) => match *f {
            TyUnit | TyAlpha(_) | TyVar(_) => {
                format!(
                    "{} -> {}",
                    pprint_type(names, f),
                    pprint_type(names, t)
                )
            }
            _ => {
                format!(
                    "({}) -> {}",
                    pprint_type(names, f),
                    pprint_type(names, t)
                )
            }
        },
        TyForall(n, y) => {
            format!("/\\ {} => {}", names[n], pprint_type(names, y))
        }
        TyError(err) => err,
    }
}
