use std::collections::{HashMap, VecDeque};

use crate::{atoms::*, multi_result::MultiResult, syntax::*};

use super::{error::*, subst_type};

pub struct Typeck(HashMap<Var, Type>, AlphaGen);

impl From<AlphaGen> for Typeck {
    fn from(gen: AlphaGen) -> Self {
        Typeck(HashMap::default(), gen)
    }
}

impl Typeck {
    pub fn typeck(&mut self, term: Term) -> TypeckResult {
        match (*term).clone() {
            TmUnit => ty::unit().into(),
            TmVar(v) => self.get(v).unwrap_or_else(|| self.next_hole()).into(),
            TmAbs(v, t, y) => {
                self.insert(v, t.clone()).typeck(y).map(|y| ty::arr(t, y))
            }
            TmApp(f, x) => (self.typeck(f) + self.typeck(x))
                .then(|(f, x)| self.assert_app(f, x)),
            TmTyAbs(n, x) => self.typeck(x).map(|x| ty::forall(n, x)),
            TmTyApp(f, t) => self.typeck(f).then(|f| self.assert_ty_app(f, t)),
            TmError => unreachable!(),
        }
    }

    fn get(&self, v: Var) -> Option<Type> {
        self.0.get(&v).cloned()
    }

    fn insert(&mut self, v: Var, t: Type) -> &mut Self {
        self.0.insert(v, t);
        self
    }

    fn assert_app(&mut self, fun: Type, arg: Type) -> TypeckResult {
        match (*fun).clone() {
            TyArrow(from, to) if from == arg => to.into(),
            TyArrow(from, to) => TypeckResult::err(to, NotEqual(from, arg)),
            _ => TypeckResult::err(self.next_hole(), NotAFunction(fun)),
        }
    }

    fn assert_ty_app(&mut self, fun: Type, arg: Type) -> TypeckResult {
        match (*fun).clone() {
            TyForall(var, inner) => subst_type(inner, arg, var).into(),
            _ => TypeckResult::err(self.next_hole(), NotAForall(fun)),
        }
    }

    fn next_hole(&mut self) -> Type {
        ty::hole(self.1.next())
    }
}

pub type TypeckResult = MultiResult<Type, (), VecDeque<TypeckError>>;
