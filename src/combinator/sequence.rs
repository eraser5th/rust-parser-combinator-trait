use crate::{parser::Parser, stream::Stream};

impl<S, P1, P2, R1, R2> Parser<S> for (P1, P2)
where
    P1: Parser<S, R = R1>,
    P2: Parser<S, R = R2>,
    S: Stream,
{
    type R = (P1::R, P2::R);
    fn parse(input: S) -> Option<(Self::R, S)> {
        let (item1, remain) = P1::parse(input)?;
        let (item2, remain) = P2::parse(remain)?;

        Some(((item1, item2), remain))
    }
}
impl<S, P1, P2, P3, R1, R2, R3> Parser<S> for (P1, P2, P3)
where
    P1: Parser<S, R = R1>,
    P2: Parser<S, R = R2>,
    P3: Parser<S, R = R3>,
    S: Stream,
{
    type R = (P1::R, P2::R, P3::R);
    fn parse(input: S) -> Option<(Self::R, S)> {
        let (item1, remain) = P1::parse(input)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P3::parse(remain)?;

        Some(((item1, item2, item3), remain))
    }
}
impl<S, P1, P2, P3, P4, R1, R2, R3, R4> Parser<S> for (P1, P2, P3, P4)
where
    P1: Parser<S, R = R1>,
    P2: Parser<S, R = R2>,
    P3: Parser<S, R = R3>,
    P4: Parser<S, R = R4>,
    S: Stream,
{
    type R = (P1::R, P2::R, P3::R, P4::R);
    fn parse(input: S) -> Option<(Self::R, S)> {
        let (item1, remain) = P1::parse(input)?;
        let (item2, remain) = P2::parse(remain)?;
        let (item3, remain) = P3::parse(remain)?;
        let (item4, remain) = P4::parse(remain)?;

        Some(((item1, item2, item3, item4), remain))
    }
}
