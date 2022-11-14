use crate::lexer::{Token, TokenStream};
use crate::parser::parser_combiner::combiner::judge;
use crate::parser::parser_combiner::traits::Parser;
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
