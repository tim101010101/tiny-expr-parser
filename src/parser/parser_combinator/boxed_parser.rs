use crate::lexer::TokenStream;
use crate::parser::parser_combiner::traits::ParserResult;
use crate::parser::parser_combiner::Parser;

pub struct BoxedParser<'input, Output> {
    pub(crate) parser: Box<dyn Parser<'input, Output> + 'input>,
}

impl<'input, Output> BoxedParser<'input, Output> {
    pub fn new<P>(parser: P) -> Self
    where
        P: Parser<'input, Output> + 'input,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

/// implement the `Parser` trait for the `BoxedParser`
/// to support chained calls
impl<'input, Output> Parser<'input, Output> for BoxedParser<'input, Output> {
    fn parse(&self, input: TokenStream) -> ParserResult<Output> {
        self.parser.parse(input)
    }
}
