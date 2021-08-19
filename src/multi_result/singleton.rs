use std::{
    collections::{HashSet, LinkedList, VecDeque},
    hash::Hash,
    marker::PhantomData,
};

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
