use std::{collections::{HashSet, LinkedList}, hash::Hash, ops::Add};

#[derive(Debug)]
pub struct MultiResult<T, E> {
    pub result: T,
    pub errors: E,
}

impl<T, E> From<T> for MultiResult<T, E>
where
    E: Default,
{
    fn from(result: T) -> Self {
        Self {
            result,
            errors: E::default(),
        }
    }
}

impl<T, U, E> Add<MultiResult<U, E>> for MultiResult<T, E>
where
    E: Semigroup,
{
    type Output = MultiResult<(T, U), E>;

    fn add(mut self, mut rhs: MultiResult<U, E>) -> Self::Output {
        MultiResult {
            result: (self.result, rhs.result),
            errors: self.errors.app(rhs.errors),
        }
    }
}

impl<T, E> From<MultiResult<T, E>> for Result<T, E>
where
    E: Neutral,
{
    fn from(result: MultiResult<T, E>) -> Self {
        if result.errors.is_neutral() {
            Ok(result.result)
        } else {
            Err(result.errors)
        }
    }
}

impl<T, E> MultiResult<T, E> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> MultiResult<U, E> {
        MultiResult {
            result: f(self.result),
            errors: self.errors,
        }
    }

    pub fn then<U>(
        mut self,
        f: impl FnOnce(T) -> MultiResult<U, E>,
    ) -> MultiResult<U, E>
    where
        E: Semigroup,
    {
        let MultiResult { result, errors } = f(self.result);
        MultiResult {
            result,
            errors: self.errors.app(errors),
        }
    }
}

pub trait Semigroup {
    fn app(self, other: Self) -> Self;
}

pub trait Neutral {
    fn is_neutral(&self) -> bool;
}

impl<T> Semigroup for Vec<T> {
    fn app(mut self, mut other: Self) -> Self {
        if self.len() < other.len() {
            return other.app(self);
        }
        self.append(&mut other);
        self
    }
}

impl<T> Neutral for Vec<T> {
    fn is_neutral(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Semigroup for HashSet<T>
where
    T: Hash + Eq,
{
    fn app(mut self, other: Self) -> Self {
        if self.len() < other.len() {
            return other.app(self);
        }
        for other in other {
            self.insert(other);
        }
        self
    }
}

impl<T> Neutral for HashSet<T> {
    fn is_neutral(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Semigroup for LinkedList<T> {
    fn app(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

impl<T> Neutral for LinkedList<T> {
    fn is_neutral(&self) -> bool {
        self.is_empty()
    }
}
