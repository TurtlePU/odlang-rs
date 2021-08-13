use std::{collections::{HashMap, HashSet}, error::Error, fmt::{Display, Formatter}, rc::Rc};

use crate::{names::{Names, Var}, parser::{
    InputTerm::{self, *},
    InputType::{self, *},
}};

pub type Term = Rc<TermData>;
pub type Type = Rc<TypeData>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TermData {
    TmUnit,
    TmVar(Var),
    TmAbs(Var, Type, Term),
    TmApp(Term, Term),
    TmTyAbs(Var, Term),
    TmTyApp(Term, Type),
    TmError(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeData {
    TyUnit,
    TyAlpha(Alpha),
    TyVar(Var),
    TyArrow(Type, Type),
    TyForall(Var, Type),
    TyError(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Alpha(usize);

impl Display for Alpha {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "_{}", self.0)
    }
}

#[derive(Default)]
pub struct AlphaGen(usize);

impl AlphaGen {
    pub fn next(&mut self) -> Alpha {
        self.0 += 1;
        Alpha(self.0)
    }
}

pub fn identify(term: InputTerm) -> Result<(Term, Names, AlphaGen), Unbound> {
    let mut ctx = Context::default();
    let term = ctx.rename_term(term);
    let (names, alpha) = ctx.terminate()?;
    Ok((term, names, alpha))
}

#[derive(Debug, Default)]
pub struct Unbound(HashSet<String>);

impl Error for Unbound {}

impl Display for Unbound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for name in &self.0 {
            writeln!(f, "Unbound name: {}", name)?;
        }
        Ok(())
    }
}

impl Unbound {
    fn report(&mut self, unbound: String) {
        self.0.insert(unbound);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Default)]
struct Stack(HashMap<String, Vec<Var>>);

impl Stack {
    fn push(&mut self, name: String, var: Var) {
        self.0.entry(name).or_default().push(var)
    }

    fn map(&self, name: &String) -> Option<Var> {
        self.0.get(name)?.last().copied()
    }

    fn pop(&mut self, name: &String) -> Option<Var> {
        self.0.get_mut(name)?.pop()
    }

    fn is_empty(&self) -> bool {
        self.0.values().all(|stack| stack.is_empty())
    }
}

#[derive(Default)]
struct Context {
    names: Names,
    alpha: AlphaGen,
    stack: Stack,
    unbound: Unbound,
}

impl Context {
    fn rename_term(&mut self, term: InputTerm) -> Term {
        match term {
            TmUnit => de::unit(),
            TmVar(x) => self.rename_var(x, de::var, de::error),
            TmApp(f, x) => de::app(self.rename_term(*f), self.rename_term(*x)),
            TmAbs(x, t, y) => self.rename_arrow(x, |sel, var| {
                de::abs(var, sel.rename_type(t), sel.rename_term(*y))
            }),
            TmTyAbs(t, x) => self
                .rename_arrow(t, |sel, var| de::ty_abs(var, sel.rename_term(*x))),
            TmTyApp(f, x) => {
                de::ty_app(self.rename_term(*f), self.rename_type(x))
            }
        }
    }

    fn rename_type(&mut self, t: InputType) -> Type {
        use de::ty;
        match t {
            TyUnit => ty::unit(),
            TyHole => ty::hole(self.alpha.next()),
            TyVar(x) => self.rename_var(x, ty::var, ty::error),
            TyArrow(from, to) => {
                ty::arr(self.rename_type(*from), self.rename_type(*to))
            }
            TyForall(param, body) => self.rename_arrow(param, |sel, var| {
                ty::forall(var, sel.rename_type(*body))
            }),
        }
    }

    fn rename_var<T>(
        &mut self,
        name: String,
        then: impl FnOnce(Var) -> T,
        err: impl FnOnce(String) -> T,
    ) -> T {
        match self.stack.map(&name) {
            Some(var) => then(var),
            None => {
                self.unbound.report(name.clone());
                err(name)
            }
        }
    }

    fn rename_arrow<T>(
        &mut self,
        name: String,
        then: impl FnOnce(&mut Self, Var) -> T,
    ) -> T {
        let var = self.names.push(name.clone());
        self.stack.push(name.clone(), var);
        let result = then(self, var);
        assert_eq!(self.stack.pop(&name), Some(var));
        result
    }

    fn terminate(self) -> Result<(Names, AlphaGen), Unbound> {
        let Context {
            names,
            alpha,
            stack,
            unbound,
        } = self;
        assert!(stack.is_empty());
        if unbound.is_empty() {
            Ok((names, alpha))
        } else {
            Err(unbound)
        }
    }
}

pub mod de {
    use super::{Term, TermData::*, Type, Var};

    pub fn unit() -> Term {
        TmUnit.into()
    }

    pub fn abs(
        param: impl Into<Var>,
        r#type: impl Into<Type>,
        body: impl Into<Term>,
    ) -> Term {
        TmAbs(param.into(), r#type.into(), body.into()).into()
    }

    pub fn app(f: impl Into<Term>, x: impl Into<Term>) -> Term {
        TmApp(f.into(), x.into()).into()
    }

    pub fn var(key: impl Into<Var>) -> Term {
        TmVar(key.into()).into()
    }

    pub fn ty_abs(param: impl Into<Var>, body: impl Into<Term>) -> Term {
        TmTyAbs(param.into(), body.into()).into()
    }

    pub fn ty_app(f: impl Into<Term>, ty: impl Into<Type>) -> Term {
        TmTyApp(f.into(), ty.into()).into()
    }

    pub fn error(name: impl Into<String>) -> Term {
        TmError(name.into()).into()
    }

    pub mod ty {
        use crate::ident::{Alpha, Type, TypeData::*, Var};

        pub fn unit() -> Type {
            TyUnit.into()
        }

        pub fn hole(num: impl Into<Alpha>) -> Type {
            TyAlpha(num.into()).into()
        }

        pub fn var(key: impl Into<Var>) -> Type {
            TyVar(key.into()).into()
        }

        pub fn arr(from: impl Into<Type>, to: impl Into<Type>) -> Type {
            TyArrow(from.into(), to.into()).into()
        }

        pub fn forall(param: impl Into<Var>, of: impl Into<Type>) -> Type {
            TyForall(param.into(), of.into()).into()
        }

        pub fn error(name: impl Into<String>) -> Type {
            TyError(name.into()).into()
        }
    }
}

impl From<usize> for Alpha {
    fn from(num: usize) -> Self {
        Self(num)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ident::{
            de::{self, ty},
            Context, Term,
        },
        parser::parse,
    };

    fn parsed(input: &str) -> Term {
        Context::default().rename_term(parse(input).unwrap())
    }

    #[test]
    fn triv() {
        assert_eq!(parsed(r"\x:().x"), de::abs(0, ty::unit(), de::var(0)));
    }

    #[test]
    fn breaks_rosser() {
        assert_eq!(
            parsed(r"\y:(). (\x:(). \y:(). x) y"),
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
