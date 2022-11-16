mod dfa;
mod tokenizer;

use tokenizer::Tokenizer;

pub use tokenizer::{Token, TokenStream};

/// Lexer
/// convert a character stream to a token stream
/// return a `Result` to indicate whether it is successful or not
/// - Ok(TokenStream)
/// - Err(ErrorMessage)
pub fn lex(code: &str) -> Result<TokenStream, String> {
    let mut tokenizer = Tokenizer::new(code.to_string());
    match tokenizer.run() {
        Ok(_) => Ok(tokenizer.token_stream()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::syntax_kind::*;

    #[test]
    fn smoke() {
        assert_eq!(
            vec![
                (OPEN_PAREN, "(".to_string()),
                (NUM, "1".to_string()),
                (PLUS, "+".to_string()),
                (NUM, "2".to_string()),
                (CLOSE_PAREN, ")".to_string()),
                (PLUS, "+".to_string()),
                (NUM, "3".to_string()),
            ],
            lex("(1 + 2) + 3").unwrap()
        );
        assert_eq!(
            vec![
                (NUM, "1".to_string()),
                (PLUS, "+".to_string()),
                (OPEN_PAREN, "(".to_string()),
                (NUM, "2".to_string()),
                (PLUS, "+".to_string()),
                (NUM, "3".to_string()),
                (CLOSE_PAREN, ")".to_string()),
            ],
            lex("1 + (2 + 3)").unwrap()
        );
        assert_eq!(
            vec![
                (OPEN_PAREN, "(".to_string()),
                (NUM, "1".to_string()),
                (CLOSE_PAREN, ")".to_string()),
                (PLUS, "+".to_string()),
                (OPEN_PAREN, "(".to_string()),
                (NUM, "2".to_string()),
                (CLOSE_PAREN, ")".to_string()),
                (PLUS, "+".to_string()),
                (OPEN_PAREN, "(".to_string()),
                (NUM, "3".to_string()),
                (CLOSE_PAREN, ")".to_string()),
            ],
            lex("(1) + (2) + (3)").unwrap()
        );
        assert_eq!(
            vec![
                (OPEN_PAREN, "(".to_string()),
                (NUM, "-1".to_string()),
                (CLOSE_PAREN, ")".to_string()),
                (PLUS, "+".to_string()),
                (OPEN_PAREN, "(".to_string()),
                (NUM, "-2".to_string()),
                (CLOSE_PAREN, ")".to_string()),
                (PLUS, "+".to_string()),
                (OPEN_PAREN, "(".to_string()),
                (NUM, "-3".to_string()),
                (CLOSE_PAREN, ")".to_string()),
            ],
            lex("(-1) + (-2) + (-3)").unwrap()
        );
    }
}
