mod lexer;
mod parser;
mod syntax_kind;
mod traversal;

use lexer::lex;
use parser::{syntax, Node};

pub use traversal::{eval, format};

/// build a AST from a expression
pub fn build_ast(expr: &str) -> Result<Node, String> {
    let root = syntax(lex(expr)?)?;
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let expr = "1 * 2 + (3 / (4  + (-5)))";
        let ast = build_ast(expr).unwrap();
        assert_eq!(-1, eval(&ast));
        assert_eq!("1 * 2 + 3 / (4 + (-5))", format(&ast));
    }
}
