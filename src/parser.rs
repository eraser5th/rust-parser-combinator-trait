pub trait Parser {
    type R;
    fn parse(s: &str) -> Option<(Self::R, &str)>;
}

pub struct Item;
impl Parser for Item {
    type R = char;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let mut chars = s.chars();
        chars.next().map(|c| (c, chars.as_str()))
    }
}

pub struct Repeat0<P> {
    _marker: std::marker::PhantomData<P>,
}
impl<P, R> Parser for Repeat0<P>
where
    P: Parser<R = R>,
    R: Clone,
{
    type R = Vec<R>;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let mut items = Vec::new();
        let mut remain = s;
        while let Some((item, new_remain)) = P::parse(remain) {
            items.push(item);
            remain = new_remain;
        }
        Some((items, remain))
    }
}

pub struct Repeat1<P> {
    _marker: std::marker::PhantomData<P>,
}
impl<P, R> Parser for Repeat1<P>
where
    P: Parser<R = R>,
    R: Clone,
{
    type R = Vec<R>;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item, remain) = P::parse(s)?;
        let mut items = vec![item];
        let mut remain = remain;
        while let Some((item, new_remain)) = P::parse(remain) {
            items.push(item);
            remain = new_remain;
        }
        Some((items, remain))
    }
}

impl<P1, P2, R> Parser for (P1, P2)
where
    P1: Parser<R = R>,
    P2: Parser<R = R>,
{
    type R = (P1::R, P2::R);
    fn parse(s: &str) -> Option<((P1::R, P2::R), &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;

        Some(((item1, item2), remain))
    }
}
impl<P1, P2, P3, R> Parser for (P1, P2, P3)
where
    P1: Parser<R = R>,
    P2: Parser<R = R>,
    P3: Parser<R = R>,
{
    type R = (P1::R, P2::R, P3::R);
    fn parse(s: &str) -> Option<((P1::R, P2::R, P3::R), &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P2::parse(remain)?;

        Some(((item1, item2, item3), remain))
    }
}
impl<P1, P2, P3, P4, R> Parser for (P1, P2, P3, P4)
where
    P1: Parser<R = R>,
    P2: Parser<R = R>,
    P3: Parser<R = R>,
    P4: Parser<R = R>,
{
    type R = (P1::R, P2::R, P3::R, P4::R);
    fn parse(s: &str) -> Option<((P1::R, P2::R, P3::R, P4::R), &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P2::parse(remain)?;
        let (item4, remain) = P2::parse(remain)?;

        Some(((item1, item2, item3, item4), remain))
    }
}

#[cfg(test)]
mod test_item_parse {
    use super::*;

    #[test]
    fn test_parse() {
        type Parser = Item;
        let input = "abc";

        let result = Parser::parse(input);

        assert_eq!(result, Some(('a', "bc")));
    }

    #[test]
    fn test_parse_empty() {
        type Parser = Item;
        let input = "";

        let result = Parser::parse(input);

        assert_eq!(result, None);
    }
}
