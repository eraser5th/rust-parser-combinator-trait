use crate::{parser::Parser, token::Token};

pub trait FromParserResult<R> {
    type Output;
    fn convert(r: R) -> Self::Output;
}

pub struct Map<P, T> {
    _marker: std::marker::PhantomData<(P, T)>,
}
impl<'a, P, T, R> Parser<'a> for Map<P, T>
where
    P: Parser<'a, R = R>,
    T: FromParserResult<R>,
{
    type R = T::Output;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item, remain) = P::parse(s)?;
        Some((T::convert(item), remain))
    }
}

pub struct ToString;
impl FromParserResult<Vec<char>> for ToString {
    type Output = String;
    fn convert(r: Vec<char>) -> Self::Output {
        r.into_iter().collect()
    }
}
impl FromParserResult<char> for ToString {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::Item;
    use crate::token::TokenIsThat;

    #[test]
    fn test_map_to_string() {
        type Parser = Map<Item, ToString>;
        let input = "a123";

        let result = Parser::parse(input);

        assert_eq!(result, Some(("a".to_string(), "123")));
    }

    fn test_map_to_string_vec() {
        type Parser = Map<crate::parser::Repeat1<Item>, ToString>;
        let input = "abc123";

        let result = Parser::parse(input);

        assert_eq!(result, Some(("abc".to_string(), "123")));
    }

    #[test]
    fn test_map_to_token() {
        type Parser = Map<Item, ToToken<'static, TokenIsThat<'a'>>>;
        let input = "a123";

        let result = Parser::parse(input);

        assert!(result.is_some());
        let (token, remain) = result.unwrap();
        assert_eq!(TokenIsThat::<'a'>::build('a'), token);
        assert_eq!(remain, "123");
    }
}
