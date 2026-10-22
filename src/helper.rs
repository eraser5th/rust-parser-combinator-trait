use crate::predict::IsThat;
use crate::{Filter, Item};

pub type CharThat<const C: char> = Filter<Item, IsThat<C>>;

mod test {
    use super::CharThat;
    use crate::*;

    #[test]
    fn test_char_that() {
        let input = "abc";
        let result = <CharThat<'a'>>::parse(input);
        assert_eq!(result, Some(('a', "bc")));
    }

    #[test]
    fn test_char_that_mismatch() {
        let input = "xbc";

        let result = <CharThat<'a'>>::parse(input);

        assert_eq!(result, None);
    }
}
