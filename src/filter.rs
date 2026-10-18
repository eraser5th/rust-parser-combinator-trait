use crate::parser::Parser;
use crate::predict::Predict;

pub struct Filter<I, P> {
    _marker: std::marker::PhantomData<(I, P)>,
}

impl<I, P, R> Parser for Filter<I, P>
where
    I: Parser<R = R>,
    P: Predict<R>,
    R: Clone,
{
    type R = R;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item, rest) = I::parse(s)?;
        P::predict(item.clone()).then_some((item, rest))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{parser::Item, predict::IsDigit};

    #[test]
    fn test_parse_digit() {
        type Parser = Filter<Item, IsDigit>;
        let input = "1abc";

        let result = Parser::parse(input);

        assert_eq!(result, Some(('1', "abc")));
    }
}
