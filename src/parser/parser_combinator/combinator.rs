use crate::lexer::TokenStream;
use crate::parser::parser_combinator::traits::Parser;

pub fn map<'input, P, Output, MapFn, NewOutput>(
    parser: P,
    map_fn: MapFn,
) -> impl Parser<'input, NewOutput>
where
    P: Parser<'input, Output>,
    MapFn: Fn(Output) -> NewOutput,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, output)| (next_input, map_fn(output)))
    }
}

pub fn and_then<'input, CurParser, CurOutput, NextFn, NextParser, NextOutput>(
    cur_parser: CurParser,
    next_fn: NextFn,
) -> impl Parser<'input, NextOutput>
where
    CurParser: Parser<'input, CurOutput>,
    NextFn: Fn(CurOutput) -> NextParser,
    NextParser: Parser<'input, NextOutput>,
{
    move |input| match cur_parser.parse(input) {
        Ok((next_input, cur_output)) => match next_fn(cur_output).parse(next_input) {
            Ok((final_input, next_output)) => Ok((final_input, next_output)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn judge<'input, P, Output, JudgeFn>(
    parser: P,
    judge_fn: JudgeFn,
) -> impl Parser<'input, Output>
where
    P: Parser<'input, Output>,
    JudgeFn: Fn(&Output) -> bool,
{
    move |input: TokenStream| match parser.parse(input.clone()) {
        Ok((next_input, output)) if judge_fn(&output) => Ok((next_input, output)),
        _ => Err(input),
    }
}

pub fn either<'input, P1, P2, Output>(parser1: P1, parser2: P2) -> impl Parser<'input, Output>
where
    P1: Parser<'input, Output>,
    P2: Parser<'input, Output>,
{
    move |input: TokenStream| match parser1.parse(input.clone()) {
        Ok((next_input, output)) => Ok((next_input, output)),
        Err(_) => parser2.parse(input),
    }
}

pub fn zero_or_more<'input, P, Output>(parser: P) -> impl Parser<'input, Vec<Output>>
where
    P: Parser<'input, Output>,
{
    move |mut input: TokenStream| {
        let mut result = Vec::new();
        while let Ok((next_input, item)) = parser.parse(input.clone()) {
            input = next_input;
            result.push(item)
        }
        Ok((input, result))
    }
}
