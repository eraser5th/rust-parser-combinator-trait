pub trait Stream: Copy {
    type Item;
    /// 先頭の1要素と、残りのストリームを返す
    fn next(self) -> Option<(Self::Item, Self)>;
}

impl Stream for &str {
    type Item = char;
    fn next(self) -> Option<(char, Self)> {
        let mut it = self.chars();
        it.next().map(|c| (c, it.as_str()))
    }
}

impl<T: Clone> Stream for &[T] {
    type Item = T;
    fn next(self) -> Option<(T, Self)> {
        self.split_first().map(|(x, rest)| (x.clone(), rest))
    }
}
