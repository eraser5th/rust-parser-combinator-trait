pub trait Token<'a, In> {
    fn build(r: In) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenIsThat<const C: char>;
impl<'a, const C: char> Token<'a, char> for TokenIsThat<C> {
    fn build(r: char) -> Self {
        assert_eq!(r, C);
        TokenIsThat
    }
}
