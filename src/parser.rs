pub trait Parser<'a, T: 'a> {
    type R;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])>;
}
