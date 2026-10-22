use crate::parser::Parser;
use crate::stream::Stream;

/// 入力の要素を1つ読む
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item;
impl<S: Stream> Parser<S> for Item {
    type R = S::Item;
    fn parse(stream: S) -> Option<(Self::R, S)> {
        Stream::next(stream)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::chars;

    #[test]
    fn test_parse() {
        type Parser = Item;
        let input = "abc";
        let result = Parser::parse(input);
        assert_eq!(result, Some(('a', "bc")));
    }

    #[test]
    fn test_parse_empty() {
        type Parser = Item;
        let input = "";
        let result = Parser::parse(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_non_char() {
        type Parser = Item;
        let input = [1, 2, 3].as_slice();
        let result = Parser::parse(input);
        assert_eq!(result, Some((1, [2, 3].as_slice())));
    }
}
