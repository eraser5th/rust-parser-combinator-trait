use crate::parser::Parser;

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
