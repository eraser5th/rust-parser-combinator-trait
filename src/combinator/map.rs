use crate::convert::FromParserResult;
use crate::parser::Parser;
use crate::stream::Stream;

pub struct Map<P, F> {
    _marker: std::marker::PhantomData<(P, F)>,
}
impl<S, P, F, R> Parser<S> for Map<P, F>
where
    P: Parser<S, R = R>,
    F: FromParserResult<R>,
    S: Stream,
{
    type R = F::Output;
    fn parse(input: S) -> Option<(Self::R, S)> {
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
    use crate::token::*;

    #[test]
    fn test_map_to_string() {
        type Parser = Map<Item, IntoString>;
        let input = "a123";

        let result = Parser::parse(input);

        assert_eq!(result, Some(("a".to_string(), "123")));
    }

    #[test]
    fn test_map_to_string_vec() {
        type Parser = Map<Repeat1<Filter<Item, Not<IsDigit>>>, IntoString>;
        let input = "abc123";

        let result = Parser::parse(input);

        assert_eq!(result, Some(("abc".to_string(), "123")));
    }

    #[test]
    fn test_map_to_token() {
        type Parser = Map<Item, ToToken<TokenIsThat<'a'>>>;
        let input = "a123";

        let result = Parser::parse(input);

        assert!(result.is_some());
        let (token, remain) = result.unwrap();
        assert_eq!(TokenIsThat::<'a'>::build('a'), token);
        assert_eq!(remain, "123");
    }
}
