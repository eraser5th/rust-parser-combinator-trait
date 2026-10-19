pub trait Parser<'a> {
    type R;
    fn parse(s: &str) -> Option<(Self::R, &str)>;
}

pub struct Item;
impl<'a> Parser<'a> for Item {
    type R = char;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let mut chars = s.chars();
        chars.next().map(|c| (c, chars.as_str()))
    }
}

pub struct Repeat0<P> {
    _marker: std::marker::PhantomData<P>,
}
impl<'a, P, R> Parser<'a> for Repeat0<P>
where
    P: Parser<'a, R = R>,
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
impl<'a, P, R> Parser<'a> for Repeat1<P>
where
    P: Parser<'a, R = R>,
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

impl<'a, P1, P2, R1, R2> Parser<'a> for (P1, P2)
where
    P1: Parser<'a, R = R1>,
    P2: Parser<'a, R = R2>,
{
    type R = (P1::R, P2::R);
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;

        Some(((item1, item2), remain))
    }
}
impl<'a, P1, P2, P3, R1, R2, R3> Parser<'a> for (P1, P2, P3)
where
    P1: Parser<'a, R = R1>,
    P2: Parser<'a, R = R2>,
    P3: Parser<'a, R = R3>,
{
    type R = (P1::R, P2::R, P3::R);
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item1, remain) = P1::parse(s)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P3::parse(remain)?;

        Some(((item1, item2, item3), remain))
    }
}
impl<'a, P1, P2, P3, P4, R1, R2, R3, R4> Parser<'a> for (P1, P2, P3, P4)
where
    P1: Parser<'a, R = R1>,
    P2: Parser<'a, R = R2>,
    P3: Parser<'a, R = R3>,
    P4: Parser<'a, R = R4>,
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
pub struct ToStr<P> {
    _marker: std::marker::PhantomData<P>,
}
impl<'a, P> Parser<'a> for ToStr<P>
where
    P: Parser<'a, R = Vec<char>>,
{
    type R = &'a str;
    fn parse(s: &str) -> Option<(Self::R, &str)> {
        let (item, remain) = P::parse(s)?;
        Some((item.iter().collect::<String>().as_str(), remain))
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
