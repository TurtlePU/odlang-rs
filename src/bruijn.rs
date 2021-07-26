use crate::{parser::InputTerm, typeck::Type};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeBruijnTerm {
    TmUnit,
    TmVar(Var),
    TmAbs(Type, Box<DeBruijnTerm>),
    TmApp(Box<DeBruijnTerm>, Box<DeBruijnTerm>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Var {
    Free(String),
    Bound(usize),
}

pub fn de_bruijn(term: InputTerm) -> DeBruijnTerm {
    Renamer::default().de_bruijn(term)
}

#[derive(Default)]
struct Renamer<'a>(Option<(&'a Renamer<'a>, String)>);

impl<'a> Renamer<'a> {
    fn de_bruijn(&'a self, term: InputTerm) -> DeBruijnTerm {
        use InputTerm::*;
        match term {
            TmUnit => de::unit(),
            TmVar(x) => de::var(self.bind(x)),
            TmApp(f, x) => de::app(self.de_bruijn(*f), self.de_bruijn(*x)),
            TmAbs(x, t, y) => de::abs(t, self.lift(x).de_bruijn(*y)),
        }
    }

    fn bind(&'a self, name: String) -> Var {
        use Var::*;
        match self.0 {
            Some((_, ref k)) if *k == name => Bound(0),
            Some((prev, _)) => match prev.bind(name) {
                Bound(i) => Bound(i + 1),
                free => free,
            },
            None => Free(name),
        }
    }

    fn lift(&'a self, new_key: String) -> Self {
        Self(Some((self, new_key)))
    }
}

pub mod de {
    use super::{
        DeBruijnTerm::{self, *},
        Var,
    };
    use crate::typeck::Type;

    pub fn unit() -> DeBruijnTerm {
        TmUnit
    }

    pub fn abs(r#type: Type, body: DeBruijnTerm) -> DeBruijnTerm {
        TmAbs(r#type, Box::new(body))
    }

    pub fn app(f: DeBruijnTerm, x: DeBruijnTerm) -> DeBruijnTerm {
        TmApp(Box::new(f), Box::new(x))
    }

    pub fn var(key: impl Into<Var>) -> DeBruijnTerm {
        TmVar(key.into())
    }
}

impl From<String> for Var {
    fn from(name: String) -> Self {
        Self::Free(name)
    }
}

impl From<usize> for Var {
    fn from(depth: usize) -> Self {
        Self::Bound(depth)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bruijn::{de, de_bruijn, DeBruijnTerm},
        parser::parse,
        typeck::ty,
    };

    fn de_parsed(input: &str) -> DeBruijnTerm {
        de_bruijn(parse(input).unwrap())
    }

    #[test]
    fn triv() {
        assert_eq!(de_parsed(r"\x:().x"), de::abs(ty::unit(), de::var(0)));
    }

    #[test]
    fn breaks_rosser() {
        assert_eq!(
            de_parsed(r"\y:(). (\x:(). \y:(). x) y"),
            de::abs(
                ty::unit(),
                de::app(
                    de::abs(ty::unit(), de::abs(ty::unit(), de::var(1))),
                    de::var(0)
                )
            )
        );
    }
}
