use crate::parser::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item;
impl<'a> Parser<'a, char> for Item {
    type R = char;
    fn parse(input: &'a [char]) -> Option<(Self::R, &'a [char])> {
        input.split_first().map(|(c, rest)| (*c, rest))
    }
}

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
}
