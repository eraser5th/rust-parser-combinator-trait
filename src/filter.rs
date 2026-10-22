use crate::parser::Parser;
use crate::predict::Predict;

pub struct Filter<I, P> {
    _marker: std::marker::PhantomData<(I, P)>,
}

impl<'a, T: 'a, I, P, R> Parser<'a, T> for Filter<I, P>
where
    I: Parser<'a, T, R = R>,
    P: Predict<R>,
    R: Clone,
{
    type R = R;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        let (item, rest) = I::parse(input)?;
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
        let input: Vec<char> = "1abc".chars().collect();
        let rest: Vec<char> = "abc".chars().collect();

        let result = Parser::parse(&input);

        assert_eq!(result, Some(('1', rest.as_slice())));
    }
}
