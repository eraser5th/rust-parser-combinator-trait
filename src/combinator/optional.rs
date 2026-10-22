use crate::{parser::Parser, stream::Stream};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Optional<P>(std::marker::PhantomData<P>);
impl<S, P> Parser<S> for Optional<P>
where
    S: Stream,
    P: Parser<S>,
{
    type R = Option<P::R>;
    fn parse(input: S) -> Option<(Self::R, S)> {
        match P::parse(input) {
            Some((r, rest)) => Some((Some(r), rest)),
            None => Some((None, input)),
        }
    }
}
