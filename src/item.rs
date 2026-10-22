use crate::combinator::filter::Filter;
use crate::parser::Parser;
use crate::predict::IsThat;

/// 入力の要素を1つ読む
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item;
impl<'a, T: Clone + 'a> Parser<'a, T> for Item {
    type R = T;
    fn parse(input: &'a [T]) -> Option<(Self::R, &'a [T])> {
        input.split_first().map(|(x, rest)| (x.clone(), rest))
    }
}

/// 指定した文字 `C` のときだけ1文字読む
pub type CharThat<const C: char> = Filter<Item, IsThat<C>>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::chars;

    #[test]
    fn test_parse() {
        type Parser = Item;
        let input = chars("abc");

        let result = Parser::parse(&input);

        assert_eq!(result, Some(('a', chars("bc").as_slice())));
    }

    #[test]
    fn test_parse_empty() {
        type Parser = Item;
        let input = chars("");

        let result = Parser::parse(&input);

        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_non_char() {
        type Parser = Item;
        let input = [1, 2, 3];

        let result = Parser::parse(&input);

        assert_eq!(result, Some((1, [2, 3].as_slice())));
    }

    #[test]
    fn test_char_that() {
        type Parser = CharThat<'a'>;
        let input = chars("abc");

        let result = Parser::parse(&input);

        assert_eq!(result, Some(('a', chars("bc").as_slice())));
    }

    #[test]
    fn test_char_that_mismatch() {
        type Parser = CharThat<'a'>;
        let input = chars("xbc");

        let result = Parser::parse(&input);

        assert_eq!(result, None);
    }
}
