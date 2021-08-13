use crate::{
    atoms::{AlphaGen, Names},
    parser::InputTerm,
    syntax::Term,
};

use self::{context::Context, unbound::Unbound};

mod context;
mod stack;
mod unbound;

pub type IdResult = Result<(Term, Names, AlphaGen), Unbound>;

pub fn identify(term: InputTerm) -> IdResult {
    let mut ctx = Context::default();
    let term = ctx.rename_term(term);
    let (names, alpha) = ctx.terminate()?;
    Ok((term, names, alpha))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parser::parse,
        syntax::{de, ty},
    };

    fn parsed(input: &str) -> IdResult {
        identify(parse(input).unwrap())
    }

    #[test]
    fn triv() {
        assert_eq!(
            parsed(r"\x:().x").unwrap().0,
            de::abs(0, ty::unit(), de::var(0))
        );
    }

    #[test]
    fn breaks_rosser() {
        assert_eq!(
            parsed(r"\y:(). (\x:(). \y:(). x) y").unwrap().0,
            de::abs(
                0,
                ty::unit(),
                de::app(
                    de::abs(1, ty::unit(), de::abs(2, ty::unit(), de::var(1))),
                    de::var(0)
                )
            )
        );
    }
}
