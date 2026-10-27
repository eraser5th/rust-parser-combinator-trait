use crate::stream::Stream;

pub trait Parser<S: Stream> {
    type R;
    fn parse(input: S) -> Option<(Self::R, S)>;
}

pub type ParserResult<P, S> = <P as Parser<S>>::R;
