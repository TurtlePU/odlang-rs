use crate::{
    parser::{InputTerm, InputType},
    var::Var,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeBruijnTerm {
    TmUnit,
    TmVar(Var),
    TmAbs(Type, Box<DeBruijnTerm>),
    TmApp(Box<DeBruijnTerm>, Box<DeBruijnTerm>),
    TmTyAbs(Box<DeBruijnTerm>),
    TmTyApp(Box<DeBruijnTerm>, Type),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    TyUnit,
    TyHole,
    TyVar(Var),
    TyArrow(Box<Type>, Box<Type>),
    TyForall(Box<Type>),
}

pub fn de_bruijn(term: InputTerm) -> DeBruijnTerm {
    Renamer::default().de_bruijn(term)
}

enum Renamer<'a> {
    Empty,
    WithValue(&'a Renamer<'a>, String),
    WithType(&'a Renamer<'a>, String),
}

impl<'a> Default for Renamer<'a> {
    fn default() -> Self {
        Renamer::Empty
    }
}

impl<'a> Renamer<'a> {
    fn de_bruijn(&'a self, term: InputTerm) -> DeBruijnTerm {
        use InputTerm::*;
        match term {
            TmUnit => de::unit(),
            TmVar(x) => de::var(self.bind(x)),
            TmApp(f, x) => de::app(self.de_bruijn(*f), self.de_bruijn(*x)),
            TmAbs(x, t, y) => {
                de::abs(self.de_bruijn_type(t), self.add_value(x).de_bruijn(*y))
            }
            TmTyAbs(t, x) => de::ty_abs(self.add_type(t).de_bruijn(*x)),
            TmTyApp(f, x) => {
                de::ty_app(self.de_bruijn(*f), self.de_bruijn_type(x))
            }
        }
    }

    fn de_bruijn_type(&'a self, t: InputType) -> Type {
        use Type::*;
        match t {
            InputType::TyUnit => TyUnit,
            InputType::TyHole => TyHole,
            InputType::TyVar(x) => TyVar(self.bind_type(x)),
            InputType::TyArrow(from, to) => TyArrow(
                Box::new(self.de_bruijn_type(*from)),
                Box::new(self.de_bruijn_type(*to)),
            ),
            InputType::TyForall(param, body) => {
                TyForall(Box::new(self.add_type(param).de_bruijn_type(*body)))
            }
        }
    }

    fn bind_type(&'a self, name: String) -> Var {
        use Renamer::*;
        use Var::*;
        match self {
            WithType(_, ref k) if *k == name => Bound(0),
            WithType(prev, _) => match prev.bind(name) {
                Bound(i) => Bound(i + 1),
                free => free,
            },
            WithValue(prev, _) => prev.bind(name),
            Empty => Free(name),
        }
    }

    fn bind(&'a self, name: String) -> Var {
        use Renamer::*;
        use Var::*;
        match self {
            WithValue(_, ref k) if *k == name => Bound(0),
            WithValue(prev, _) => match prev.bind(name) {
                Bound(i) => Bound(i + 1),
                free => free,
            },
            WithType(prev, _) => prev.bind(name),
            Empty => Free(name),
        }
    }

    fn add_value(&'a self, new_key: String) -> Self {
        Self::WithValue(self, new_key)
    }

    fn add_type(&'a self, new_type: String) -> Self {
        Self::WithType(self, new_type)
    }
}

pub mod de {
    use super::{
        DeBruijnTerm::{self, *},
        Type, Var,
    };

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

    pub fn ty_abs(body: DeBruijnTerm) -> DeBruijnTerm {
        TmTyAbs(Box::new(body))
    }

    pub fn ty_app(f: DeBruijnTerm, ty: Type) -> DeBruijnTerm {
        TmTyApp(Box::new(f), ty)
    }

    pub mod ty {
        use super::{Type, Var};

        pub fn unit() -> Type {
            Type::TyUnit
        }

        pub fn hole() -> Type {
            Type::TyHole
        }

        pub fn var(key: impl Into<Var>) -> Type {
            Type::TyVar(key.into())
        }

        pub fn arr(from: Type, to: Type) -> Type {
            Type::TyArrow(Box::new(from), Box::new(to))
        }

        pub fn forall(of: Type) -> Type {
            Type::TyForall(Box::new(of))
        }
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
        bruijn::{
            de::{self, ty},
            de_bruijn, DeBruijnTerm,
        },
        parser::parse,
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
