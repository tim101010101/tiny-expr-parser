use crate::lexer::{Token, TokenStream};
use crate::parser::parser_combinator::combinator::judge;
use crate::parser::parser_combinator::traits::Parser;
use crate::syntax_kind::SyntaxKind;

pub fn atom<'input>() -> impl Parser<'input, Token> {
    move |input: TokenStream| {
        let mut it = input.iter();
        match it.next() {
            Some(next) => Ok((
                input.iter().skip(1).map(|t| t.to_owned()).collect(),
                next.to_owned(),
            )),
            None => Err(input),
        }
    }
}

pub fn single_token(expect: SyntaxKind) -> impl Parser<'static, Token> {
    judge(atom(), move |(kind, _)| *kind == expect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax_kind::{NUM, PLUS};

    #[test]
    fn test_atom() {
        let input = vec![(NUM, "1".to_string()), (NUM, "2".to_string())];
        assert_eq!(
            Ok((vec![(NUM, "2".to_string())], (NUM, "1".to_string()))),
            atom().parse(input)
        );
    }

    #[test]
    fn test_single_token() {
        let input = vec![(PLUS, "+".to_string())];
        assert_eq!(
            Ok((vec![], (PLUS, "+".to_string()))),
            single_token(PLUS).parse(input)
        )
    }
}
