pub trait Parser<'a, T: 'a> {
    type R;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item;
impl<'a> Parser<'a, char> for Item {
    type R = char;
    fn parse(input: &'a [char]) -> Option<(Self::R, &'a [char])> {
        input.split_first().map(|(c, rest)| (*c, rest))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repeat0<P>(std::marker::PhantomData<P>);
impl<'a, T: 'a, P, R> Parser<'a, T> for Repeat0<P>
where
    P: Parser<'a, T, R = R>,
    R: Clone,
{
    type R = Vec<R>;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        let mut items = Vec::new();
        let mut remain = input;
        while let Some((item, new_remain)) = P::parse(remain) {
            items.push(item);
            remain = new_remain;
        }
        Some((items, remain))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repeat1<P>(std::marker::PhantomData<P>);
impl<'a, T: 'a, P, R> Parser<'a, T> for Repeat1<P>
where
    P: Parser<'a, T, R = R>,
    R: Clone,
{
    type R = Vec<R>;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        let (item, remain) = P::parse(input)?;
        let mut items = vec![item];
        let mut remain = remain;
        while let Some((item, new_remain)) = P::parse(remain) {
            items.push(item);
            remain = new_remain;
        }
        Some((items, remain))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Optional<P>(std::marker::PhantomData<P>);
impl<'a, T: 'a, P: Parser<'a, T>> Parser<'a, T> for Optional<P> {
    type R = Option<P::R>;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        match P::parse(input) {
            Some((r, rest)) => Some((Some(r), rest)),
            None => Some((None, input)), // 失敗したら何も読まずに成功扱い
        }
    }
}

impl<'a, T: 'a, P1, P2, R1, R2> Parser<'a, T> for (P1, P2)
where
    P1: Parser<'a, T, R = R1>,
    P2: Parser<'a, T, R = R2>,
{
    type R = (P1::R, P2::R);
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        let (item1, remain) = P1::parse(input)?;
        let (item2, remain) = P2::parse(remain)?;

        Some(((item1, item2), remain))
    }
}
impl<'a, T: 'a, P1, P2, P3, R1, R2, R3> Parser<'a, T> for (P1, P2, P3)
where
    P1: Parser<'a, T, R = R1>,
    P2: Parser<'a, T, R = R2>,
    P3: Parser<'a, T, R = R3>,
{
    type R = (P1::R, P2::R, P3::R);
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        let (item1, remain) = P1::parse(input)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P3::parse(remain)?;

        Some(((item1, item2, item3), remain))
    }
}
impl<'a, T: 'a, P1, P2, P3, P4, R1, R2, R3, R4> Parser<'a, T> for (P1, P2, P3, P4)
where
    P1: Parser<'a, T, R = R1>,
    P2: Parser<'a, T, R = R2>,
    P3: Parser<'a, T, R = R3>,
    P4: Parser<'a, T, R = R4>,
{
    type R = (P1::R, P2::R, P3::R, P4::R);
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        let (item1, remain) = P1::parse(input)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P3::parse(remain)?;
        let (item4, remain) = P4::parse(remain)?;

        Some(((item1, item2, item3, item4), remain))
    }
}

#[cfg(test)]
mod test_item_parse {
    use super::*;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_parse() {
        type Parser = Item;
        let input = chars("abc");

        let result = Parser::parse(&input);

        assert_eq!(result, Some(('a', chars("bc").as_slice())));
    }

    #[test]
    fn test_parse_empty() {
        type Parser = Item;
        let input = chars("");

        let result = Parser::parse(&input);

        assert_eq!(result, None);
    }
}
