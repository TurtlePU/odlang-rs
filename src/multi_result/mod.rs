mod neutral;
mod semigroup;
mod singleton;

use std::ops::Add;

pub use neutral::*;
pub use semigroup::*;
pub use singleton::*;

#[derive(Debug)]
pub struct MultiResult<T, S, E> {
    pub result: T,
    pub state: S,
    pub errors: E,
}

impl<T, S, E> From<T> for MultiResult<T, S, E>
where
    S: Default,
    E: Default,
{
    fn from(result: T) -> Self {
        Self::ok(result, empty())
    }
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

impl<T, E> From<MultiResult<T, (), E>> for Result<T, E>
where
    E: Neutral,
{
    fn from(
        MultiResult {
            result,
            state: _,
            errors,
        }: MultiResult<T, (), E>,
    ) -> Self {
        if errors.is_neutral() {
            Ok(result)
        } else {
            Err(errors)
        }
    }
}

impl<T, S, E> MultiResult<T, S, E> {
    pub fn ok<S1>(result: T, state: S1) -> MultiResult<T, S, E>
    where
        S: Singleton<S1>,
        E: Default,
    {
        MultiResult {
            result,
            state: S::single(state),
            errors: E::default(),
        }
    }

    pub fn err<E1>(result: T, error: E1) -> MultiResult<T, S, E>
    where
        S: Default,
        E: Singleton<E1>,
    {
        MultiResult {
            result,
            state: S::default(),
            errors: E::single(error),
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> MultiResult<U, S, E> {
        MultiResult {
            result: f(self.result),
            state: self.state,
            errors: self.errors,
        }
    }

    pub fn then<U>(
        self,
        f: impl FnOnce(T) -> MultiResult<U, S, E>,
    ) -> MultiResult<U, S, E>
    where
        S: Semigroup,
        E: Semigroup,
    {
        let MultiResult {
            result,
            state,
            errors,
        } = f(self.result);
        MultiResult {
            result,
            state: self.state.app(state),
            errors: self.errors.app(errors),
        }
    }
}
