pub trait Predict<R> {
    fn predict(s: R) -> bool;
}

pub struct And<P1, P2> {
    _marker: std::marker::PhantomData<(P1, P2)>,
}
impl<P1, P2, R> Predict<R> for And<P1, P2>
where
    P1: Predict<R>,
    P2: Predict<R>,
    R: Clone,
{
    fn predict(s: R) -> bool {
        P1::predict(s.clone()) && P2::predict(s)
    }
}

pub struct Or<P1, P2> {
    _marker: std::marker::PhantomData<(P1, P2)>,
}
impl<P1, P2, R> Predict<R> for Or<P1, P2>
where
    P1: Predict<R>,
    P2: Predict<R>,
    R: Clone,
{
    fn predict(s: R) -> bool {
        P1::predict(s.clone()) || P2::predict(s)
    }
}

pub struct Not<P> {
    _marker: std::marker::PhantomData<P>,
}
impl<P, R> Predict<R> for Not<P>
where
    P: Predict<R>,
{
    fn predict(s: R) -> bool {
        !P::predict(s)
    }
}

pub struct IsDigit;
impl Predict<char> for IsDigit {
    fn predict(c: char) -> bool {
        c.is_ascii_digit()
    }
}

pub struct IsNonZeroDigit;
impl Predict<char> for IsNonZeroDigit {
    fn predict(c: char) -> bool {
        c != '0' && c.is_ascii_digit()
    }
}

pub struct IsThat<const V: char>;
impl<const V: char> Predict<char> for IsThat<V> {
    fn predict(c: char) -> bool {
        c == V
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_digit() {
        let input: char = '1';
        let result = IsDigit::predict(input);
        assert!(result);
    }
    #[test]
    fn test_is_digit_0() {
        let input: char = '0';
        let result = IsDigit::predict(input);
        assert!(result);
    }
    #[test]
    fn test_is_non_zero_digit_0() {
        let input: char = '0';
        let result = IsNonZeroDigit::predict(input);
        assert!(!result);
    }
}
