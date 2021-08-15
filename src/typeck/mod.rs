mod error;
mod state;

use std::collections::VecDeque;

use crate::{atoms::*, syntax::*};

use self::{error::TypeckError, state::Typeck};

pub fn typeck(
    gen: AlphaGen,
    term: Term,
) -> Result<Type, VecDeque<TypeckError>> {
    Typeck::from(gen).typeck(term).into()
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
        TyError(_) => unreachable!(),
    }
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
