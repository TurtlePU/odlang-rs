use std::{
    collections::{HashSet, LinkedList, VecDeque},
    hash::Hash,
};

pub trait Semigroup {
    fn app(self, other: Self) -> Self;
}

impl Semigroup for () {
    fn app(self, other: Self) -> Self {}
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
