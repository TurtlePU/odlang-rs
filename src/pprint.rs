use itertools::Itertools;

use crate::{
    intern::{Context, Term, TermData::*, Type, TypeData::*},
    typeck::TypeckError,
};

pub fn pprint(ctx: &Context, term: Term) -> String {
    match (*term).clone() {
        TmUnit => "()".into(),
        TmVar(var) => ctx.name(var),
        TmAbs(n, t, y) => {
            format!(
                "\\{}: {}. {}",
                ctx.name(n),
                pprint_type(ctx, t),
                pprint(ctx, y)
            )
        }
        TmApp(f, x) => match *f {
            TmAbs(_, _, _) => {
                format!("({}) {}", pprint(ctx, f), pprint(ctx, x))
            }
            _ => format!("{} {}", pprint(ctx, f), pprint(ctx, x)),
        },
        TmTyAbs(n, y) => format!("/\\ {}. {}", ctx.name(n), pprint(ctx, y)),
        TmTyApp(f, x) => match *f {
            TmTyAbs(_, _) => {
                format!("({}) [{}]", pprint(ctx, f), pprint_type(ctx, x))
            }
            _ => format!("{} [{}]", pprint(ctx, f), pprint_type(ctx, x)),
        },
    }
}

pub fn pprint_errors(ctx: &Context, errs: Vec<TypeckError>) -> String {
    errs.into_iter().map(|err| pprint_error(ctx, err)).join("\n")
}

fn pprint_error(ctx: &Context, err: TypeckError) -> String {
    use TypeckError::*;
    match err {
        NotEqual(a, b) => format!(
            "Types should be equal: '{}', '{}'",
            pprint_type(ctx, a),
            pprint_type(ctx, b)
        ),
        NotAFunction(f) => {
            format!("Must be a function: '{}'", pprint_type(ctx, f))
        }
        NotAForall(f) => format!("Must be a forall: '{}'", pprint_type(ctx, f)),
    }
}

fn pprint_type(ctx: &Context, ty: Type) -> String {
    match (*ty).clone() {
        TyUnit => "()".into(),
        TyHole => "_".into(),
        TyVar(var) => ctx.name(var),
        TyArrow(f, t) => match *f {
            TyUnit | TyHole | TyVar(_) => {
                format!("{} -> {}", pprint_type(ctx, f), pprint_type(ctx, t))
            }
            _ => {
                format!("({}) -> {}", pprint_type(ctx, f), pprint_type(ctx, t))
            }
        },
        TyForall(n, y) => {
            format!("/\\ {} => {}", ctx.name(n), pprint_type(ctx, y))
        }
    }
}
