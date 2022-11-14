use crate::{
    lexer::TokenStream,
    parser::{grammar::expr, parser_combinator::Parser},
};

mod grammar;
mod node;
mod parser_combinator;

pub use node::Node;

/// Parser
/// build a AST from a token stream
/// return a `Result` to indicate whether it is successful or not
/// - Ok(Root)
/// - Err(ErrorMessage)
pub fn syntax(tokens: TokenStream) -> Result<Node, String> {
    match expr().parse(tokens) {
        Ok((_, n)) => Ok(n),
        Err(output) => Err(format!(
            "panic at parsing `{}`",
            output.iter().fold(String::new(), |mut res, (_, cur)| {
                res.push_str(cur);
                res
            })
        )),
    }
}
