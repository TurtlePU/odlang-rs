use std::{
    collections::{HashSet, LinkedList, VecDeque},
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Shr},
};

#[derive(Debug)]
pub struct MultiResult<T, S, E> {
    pub result: T,
    pub state: S,
    pub errors: E,
}

impl<T, U, S, E> Add<MultiResult<U, S, E>> for MultiResult<T, S, E>
where
    S: Semigroup,
    E: Semigroup,
{
    type Output = MultiResult<(T, U), S, E>;

    fn add(self, rhs: MultiResult<U, S, E>) -> Self::Output {
        MultiResult {
            result: (self.result, rhs.result),
            state: self.state.app(rhs.state),
            errors: self.errors.app(rhs.errors),
        }
    }
}

impl<T, U, R, S, E, F> Shr<F> for MultiResult<T, R, E>
where
    F: FnOnce(R) -> MultiResult<U, S, E>,
    E: Semigroup,
{
    type Output = MultiResult<(T, U), S, E>;

    fn shr(self, rhs: F) -> Self::Output {
        let rhs = rhs(self.state);
        MultiResult {
            result: (self.result, rhs.result),
            state: rhs.state,
            errors: self.errors.app(rhs.errors),
        }
    }
}

impl<T, S, E> MultiResult<T, S, E> {
    pub fn new<E1>(result: T, state: S, error: E1) -> Self
    where
        E: Singleton<E1>,
    {
        MultiResult {
            result,
            state,
            errors: E::single(error),
        }
    }

    pub fn ok(result: T, state: S) -> Self
    where
        E: Default,
    {
        Self::new(result, state, empty())
    }

    pub fn err<E1>(state: S, error: E1) -> Self
    where
        T: ErrValue,
        E: Singleton<E1>,
    {
        Self::new(T::err_value(), state, error)
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> MultiResult<U, S, E> {
        MultiResult {
            result: f(self.result),
            state: self.state,
            errors: self.errors,
        }
    }

    pub fn then<U, F>(self, f: impl FnOnce(T) -> F) -> MultiResult<U, S, E>
    where
        E: Semigroup,
        F: FnOnce(S) -> MultiResult<U, S, E>,
    {
        let MultiResult {
            result,
            state,
            errors,
        } = f(self.result)(self.state);
        MultiResult {
            result,
            state,
            errors: self.errors.app(errors),
        }
    }
}

impl<T, E> From<T> for MultiResult<T, (), E>
where
    E: Default,
{
    fn from(result: T) -> Self {
        Self::new(result, (), empty())
    }
}

pub fn pipe<T, U, S, E>(
    first: impl FnOnce(S) -> MultiResult<T, S, E>,
    second: impl FnOnce(S) -> MultiResult<U, S, E>,
) -> impl FnOnce(S) -> MultiResult<(T, U), S, E>
where
    E: Semigroup,
{
    move |state| first(state) >> second
}

pub fn fmap<T, U, S, E>(
    gen: impl FnOnce(S) -> MultiResult<T, S, E>,
    map: impl FnOnce(T) -> U,
) -> impl FnOnce(S) -> MultiResult<U, S, E>
where
    E: Semigroup,
{
    move |state| gen(state).map(map)
}

pub fn fthen<T, U, S, E, F>(
    gen: impl FnOnce(S) -> MultiResult<T, S, E>,
    then: impl FnOnce(T) -> F,
) -> impl FnOnce(S) -> MultiResult<U, S, E>
where
    F: FnOnce(S) -> MultiResult<U, S, E>,
    E: Semigroup,
{
    move |state| gen(state).then(then)
}

pub trait ErrValue {
    fn err_value() -> Self;
}

pub trait Semigroup {
    fn app(self, other: Self) -> Self;
}

impl Semigroup for () {
    fn app(self, _: Self) -> Self {}
}

impl<T, U> Semigroup for (T, U)
where
    T: Semigroup,
    U: Semigroup,
{
    fn app(self, other: Self) -> Self {
        (self.0.app(other.0), self.1.app(other.1))
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

impl<T> Semigroup for LinkedList<T> {
    fn app(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

impl<T> Semigroup for VecDeque<T> {
    fn app(mut self, mut other: Self) -> Self {
        if self.len() < other.len() {
            for x in self.into_iter().rev() {
                other.push_front(x);
            }
            other
        } else {
            self.append(&mut other);
            self
        }
    }
}

pub trait Singleton<T> {
    fn single(elem: T) -> Self;
}

pub struct Empty<T>(PhantomData<T>);

pub const fn empty<T>() -> Empty<T> {
    Empty(PhantomData)
}

impl<T> Singleton<Empty<T>> for T
where
    T: Default,
{
    fn single(_: Empty<T>) -> Self {
        Self::default()
    }
}

impl<T, U, T1, U1> Singleton<(T1, U1)> for (T, U)
where
    T: Singleton<T1>,
    U: Singleton<U1>,
{
    fn single((t, u): (T1, U1)) -> Self {
        (T::single(t), U::single(u))
    }
}

impl<T> Singleton<T> for VecDeque<T> {
    fn single(elem: T) -> Self {
        let mut result = Self::new();
        result.push_back(elem);
        result
    }
}

impl<T> Singleton<T> for HashSet<T>
where
    T: Hash + Eq,
{
    fn single(elem: T) -> Self {
        let mut result = Self::new();
        result.insert(elem);
        result
    }
}

impl<T> Singleton<T> for LinkedList<T> {
    fn single(elem: T) -> Self {
        let mut result = Self::new();
        result.push_back(elem);
        result
    }
}
