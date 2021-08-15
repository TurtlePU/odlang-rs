use std::{
    collections::{HashSet, LinkedList, VecDeque},
    hash::Hash,
};

pub trait Singleton<T> {
    fn single(elem: T) -> Self;
}

impl<T> Singleton<T> for T {
    fn single(elem: T) -> Self {
        elem
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
