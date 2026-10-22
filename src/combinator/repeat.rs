use crate::{parser::Parser, stream::Stream};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repeat0<P>(std::marker::PhantomData<P>);
impl<S, P, R> Parser<S> for Repeat0<P>
where
    P: Parser<S, R = R>,
    R: Clone,
    S: Stream,
{
    type R = Vec<R>;
    fn parse(input: S) -> Option<(Self::R, S)> {
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
impl<S, P, R> Parser<S> for Repeat1<P>
where
    P: Parser<S, R = R>,
    R: Clone,
    S: Stream,
{
    type R = Vec<R>;
    fn parse(input: S) -> Option<(Self::R, S)> {
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
