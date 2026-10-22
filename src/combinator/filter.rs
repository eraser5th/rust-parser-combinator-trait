use crate::parser::Parser;
use crate::predict::{Not, Predict};
use crate::stream::Stream;
use crate::Item;

pub struct Filter<I, P> {
    _marker: std::marker::PhantomData<(I, P)>,
}

impl<S, Pa, Pr, R> Parser<S> for Filter<Pa, Pr>
where
    Pa: Parser<S, R = R>,
    Pr: Predict<R>,
    R: Clone,
    S: Stream,
{
    type R = R;
    fn parse(input: S) -> Option<(Self::R, S)> {
        let (item, rest) = Pa::parse(input)?;
        Pr::predict(item.clone()).then_some((item, rest))
    }
}

pub type ItemThat<P> = Filter<Item, P>;
pub type ItemThatNot<P> = Filter<Item, Not<P>>;

#[cfg(test)]
mod test {
    use super::*;

    use crate::{item::Item, predict::IsDigit};

    #[test]
    fn test_parse_digit() {
        type Parser = Filter<Item, IsDigit>;
        let input = "1abc";

        let result = Parser::parse(input);

        assert_eq!(result, Some(('1', "abc")))
    }
}
