use std::{
    collections::{HashSet, LinkedList, VecDeque},
    hash::Hash,
    marker::PhantomData,
    ops::Add,
};

#[derive(Debug)]
pub struct MultiResult<R, C> {
    pub result: R,
    pub collect: C,
}

impl<Q, R, C> Add<MultiResult<R, C>> for MultiResult<Q, C>
where
    C: Semigroup,
{
    type Output = MultiResult<(Q, R), C>;

    fn add(self, rhs: MultiResult<R, C>) -> Self::Output {
        MultiResult {
            result: (self.result, rhs.result),
            collect: self.collect.app(rhs.collect),
        }
    }
}

impl<R, C> MultiResult<R, C> {
    pub fn new<I>(result: R, item: I) -> Self
    where
        C: Singleton<I>,
    {
        MultiResult {
            result,
            collect: C::single(item),
        }
    }

    pub fn item<I>(item: I) -> Self
    where
        R: Default,
        C: Singleton<I>,
    {
        Self::new(R::default(), item)
    }

    pub fn map<Q>(self, f: impl FnOnce(R) -> Q) -> MultiResult<Q, C> {
        MultiResult {
            result: f(self.result),
            collect: self.collect,
        }
    }

    pub fn then<Q>(
        self,
        f: impl FnOnce(R) -> MultiResult<Q, C>,
    ) -> MultiResult<Q, C>
    where
        C: Semigroup,
    {
        let mut result = f(self.result);
        result.collect = self.collect.app(result.collect);
        result
    }
}

impl<R, C> From<R> for MultiResult<R, C>
where
    C: Default,
{
    fn from(result: R) -> Self {
        Self::new(result, empty())
    }
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
