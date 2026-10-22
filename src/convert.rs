use std::marker::PhantomData;

use crate::token::Token;

pub trait FromParserResult<R> {
    type Output;
    fn convert(r: R) -> Self::Output;
}

pub struct IntoString;
impl FromParserResult<Vec<char>> for IntoString {
    type Output = String;
    fn convert(r: Vec<char>) -> Self::Output {
        r.into_iter().collect()
    }
}
impl FromParserResult<char> for IntoString {
    type Output = String;
    fn convert(r: char) -> Self::Output {
        r.to_string()
    }
}

pub struct Into<T>(PhantomData<T>);
impl<In, T: From<In>> FromParserResult<In> for Into<T> {
    type Output = T;
    fn convert(r: In) -> T {
        T::from(r)
    }
}

pub struct Const<T>(PhantomData<T>);
impl<In, T: Default> FromParserResult<In> for Const<T> {
    type Output = T;
    fn convert(_: In) -> T {
        T::default()
    }
}

pub struct ToToken<T> {
    _marker: PhantomData<T>,
}
impl<T, In> FromParserResult<In> for ToToken<T>
where
    T: Token<In>,
{
    type Output = T;
    fn convert(r: In) -> Self::Output {
        T::build(r)
    }
}
