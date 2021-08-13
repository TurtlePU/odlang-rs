use std::ops::Add;

use crate::syntax::Type;

use super::error::TypeckError;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeckResult<T = Type>(T, Vec<TypeckError>);

impl<T> From<T> for TypeckResult<T> {
    fn from(value: T) -> Self {
        Self(value, vec![])
    }
}

impl<T> TypeckResult<T> {
    pub fn new(result: T, error: TypeckError) -> TypeckResult<T> {
        TypeckResult(result, vec![error])
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> TypeckResult<U> {
        TypeckResult(f(self.0), self.1)
    }

    pub fn then<U>(
        self,
        f: impl FnOnce(T) -> TypeckResult<U>,
    ) -> TypeckResult<U> {
        let TypeckResult(x, mut errors) = self;
        let TypeckResult(x, mut new_errors) = f(x);
        errors.append(&mut new_errors);
        TypeckResult(x, errors)
    }
}

impl<T, U> Add<TypeckResult<U>> for TypeckResult<T> {
    type Output = TypeckResult<(T, U)>;

    fn add(self, rhs: TypeckResult<U>) -> Self::Output {
        self.then(|x| rhs.map(|y| (x, y)))
    }
}

impl<T> From<TypeckResult<T>> for Result<T, Vec<TypeckError>> {
    fn from(TypeckResult(result, errors): TypeckResult<T>) -> Self {
        if errors.is_empty() {
            Ok(result)
        } else {
            Err(errors)
        }
    }
}
