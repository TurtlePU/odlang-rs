pub use crate::{alpha::*, coordinates::*, multi_result::*, names::*};

pub trait Pair<T> {
    type Output;

    fn pair(self, other: T) -> Self::Output;
}

impl<T, U, E> Pair<Result<U, E>> for Result<T, E>
where
    E: Semigroup,
{
    type Output = Result<(T, U), E>;

    fn pair(self, other: Result<U, E>) -> Self::Output {
        match (self, other) {
            (Ok(left), Ok(right)) => Ok((left, right)),
            (Err(err), Ok(_)) | (Ok(_), Err(err)) => Err(err),
            (Err(left), Err(right)) => Err(left.app(right)),
        }
    }
}
