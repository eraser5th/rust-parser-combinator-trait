use crate::convert::FromParserResult;
use crate::parser::Parser;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::combinator::{filter::Filter, repeat::Repeat1};
    use crate::convert::{IntoString, ToToken};
    use crate::item::Item;
    use crate::predict::{IsDigit, Not};
    use crate::test_util::chars;
    use crate::token::*;

    #[test]
    fn test_map_to_string() {
        type Parser = Map<Item, IntoString>;
        let input = chars("a123");

        let result = Parser::parse(&input);

        assert_eq!(result, Some(("a".to_string(), chars("123").as_slice())));
    }

    #[test]
    fn test_map_to_string_vec() {
        type Parser = Map<Repeat1<Filter<Item, Not<IsDigit>>>, IntoString>;
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
