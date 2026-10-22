use crate::predict::IsThat;
use crate::{Filter, Item};

pub type CharThat<const C: char> = Filter<Item, IsThat<C>>;

mod test {
    use super::*;
    use crate::*;

    #[test]
    fn test_char_that() {
        let input = "abc";
        let result = <CharThat<'a'> as Parser<&str>>::parse(input);
        assert_eq!(result, Some(('a', "bc")));
    }

    #[test]
    fn test_char_that_mismatch() {
        let input = "xbc";

        let result = <CharThat<'a'> as Parser<&str>>::parse(input);

        assert_eq!(result, None);
    }
}
