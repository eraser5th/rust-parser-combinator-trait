use crate::parser::Parser;

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
