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

pub struct ToToken<'a, T> {
    _marker: std::marker::PhantomData<&'a T>,
}
impl<'a, T, In> FromParserResult<In> for ToToken<'a, T>
where
    T: Token<'a, In>,
{
    type Output = T;
    fn convert(r: In) -> Self::Output {
        T::build(r)
    }
}
