use crate::{parser::Parser, token::Token};

pub trait FromParserResult<R> {
    type Output;
    fn convert(r: R) -> Self::Output;
}

pub struct Map<P, F> {
    _marker: std::marker::PhantomData<(P, F)>,
}
impl<'a, T: 'a, P, F, R> Parser<'a, T> for Map<P, F>
where
    P: Parser<'a, T, R = R>,
    F: FromParserResult<R>,
{
    type R = F::Output;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        let (item, remain) = P::parse(input)?;
        Some((F::convert(item), remain))
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
    use crate::filter::Filter;
    use crate::parser::*;
    use crate::predict::{IsDigit, Not};
    use crate::token::*;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_map_to_string() {
        type Parser = Map<Item, ToString>;
        let input = chars("a123");

        let result = Parser::parse(&input);

        assert_eq!(result, Some(("a".to_string(), chars("123").as_slice())));
    }

    #[test]
    fn test_map_to_string_vec() {
        type Parser = Map<Repeat1<Filter<Item, Not<IsDigit>>>, ToString>;
        let input = chars("abc123");

        let result = Parser::parse(&input);

        assert_eq!(result, Some(("abc".to_string(), chars("123").as_slice())));
    }

    #[test]
    fn test_map_to_token() {
        type Parser = Map<Item, ToToken<'static, TokenIsThat<'a'>>>;
        let input = chars("a123");

        let result = Parser::parse(&input);

        assert!(result.is_some());
        let (token, remain) = result.unwrap();
        assert_eq!(TokenIsThat::<'a'>::build('a'), token);
        assert_eq!(remain, chars("123").as_slice());
    }
}
