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

impl<P1, P2, R1, R2> Parser for (P1, P2)
where
    P1: Parser<R = R1>,
    P2: Parser<R = R2>,
{
    type R = (P1::R, P2::R);
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;

        Some(((item1, item2), remain))
    }
}
impl<P1, P2, P3, R1, R2, R3> Parser for (P1, P2, P3)
where
    P1: Parser<R = R1>,
    P2: Parser<R = R2>,
    P3: Parser<R = R3>,
{
    type R = (P1::R, P2::R, P3::R);
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P3::parse(remain)?;

        Some(((item1, item2, item3), remain))
    }
}
impl<P1, P2, P3, P4, R1, R2, R3, R4> Parser for (P1, P2, P3, P4)
where
    P1: Parser<R = R1>,
    P2: Parser<R = R2>,
    P3: Parser<R = R3>,
    P4: Parser<R = R4>,
{
    type R = (P1::R, P2::R, P3::R, P4::R);
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P3::parse(remain)?;
        let (item4, remain) = P4::parse(remain)?;

        Some(((item1, item2, item3, item4), remain))
    }
}

struct Join<P, R> {
    _marker: std::marker::PhantomData<(P, R)>,
}
impl<P, PR, R, ITEM> Parser for Join<P, R>
where
    P: Parser<R = PR>,
    PR: IntoIterator<Item = ITEM>,
    R: FromIterator<<P::R as IntoIterator>::Item>,
{
    type R = R;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (items, remain) = P::parse(s)?;
        Some((items.into_iter().collect(), remain))
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
