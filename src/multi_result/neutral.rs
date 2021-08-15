use std::collections::{HashSet, LinkedList, VecDeque};

pub trait Neutral {
    fn is_neutral(&self) -> bool;
}

impl Neutral for () {
    fn is_neutral(&self) -> bool {
        true
    }
}

impl<T, U> Neutral for (T, U)
where
    T: Neutral,
    U: Neutral,
{
    fn is_neutral(&self) -> bool {
        self.0.is_neutral() && self.1.is_neutral()
    }
}

impl<T> Neutral for HashSet<T> {
    fn is_neutral(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Neutral for LinkedList<T> {
    fn is_neutral(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Neutral for VecDeque<T> {
    fn is_neutral(&self) -> bool {
        self.is_empty()
    }
}
