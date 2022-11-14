use crate::lexer::TokenStream;
use crate::parser::parser_combinator::boxed_parser::BoxedParser;
use crate::parser::parser_combinator::{and_then, either, map};

/// the result of once parsing
/// - Ok(Next Input, Current Output)
/// - Err(Current Input)
pub type ParserResult<Output> = Result<(TokenStream, Output), TokenStream>;

pub trait Parser<'input, Output> {
    fn parse(&self, input: TokenStream) -> ParserResult<Output>;

    fn map<MapFn, NewOutput>(self, map_fn: MapFn) -> BoxedParser<'input, NewOutput>
    where
        Self: Sized + 'input,
        Output: 'input,
        NewOutput: 'input,
        MapFn: Fn(Output) -> NewOutput + 'input,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn and_then<NextFn, NextParser, NextOutput>(
        self,
        next_fn: NextFn,
    ) -> BoxedParser<'input, NextOutput>
    where
        Self: Sized + 'input,
        Output: 'input,
        NextParser: Parser<'input, NextOutput> + 'input,
        NextFn: Fn(Output) -> NextParser + 'input,
        NextOutput: 'input,
    {
        BoxedParser::new(and_then(self, next_fn))
    }

    fn or<OtherParser>(self, other_parser: OtherParser) -> BoxedParser<'input, Output>
    where
        Self: Sized + 'input,
        Output: 'input,
        OtherParser: Parser<'input, Output> + 'input,
    {
        BoxedParser::new(either(self, other_parser))
    }
}

/// implement the `Parser` trait for all the `Parser-Like` function
///
/// `Parser-Like` function:
///
///     Fn(TokenStream) -> ParserResult<Output>
///
/// # Example
///
/// ```rust
/// fn get_a_parser_like_function() -> impl Parser<i32> {
///     |input: TokenStream| {
///         Ok((input, 666))
///     }
/// }
/// ```
impl<'input, Output, F> Parser<'input, Output> for F
where
    F: Fn(TokenStream) -> ParserResult<Output>,
{
    fn parse(&self, input: TokenStream) -> ParserResult<Output> {
        self(input)
    }
}
