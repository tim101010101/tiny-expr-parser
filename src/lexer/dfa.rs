use crate::{
    lexer::states::{get_terminator_judgement, get_transition, ERROR, OPERATOR, START},
    syntax_kind::{SyntaxKind, NUM},
    token,
};

/// type definition of the token
///
/// kind: token.0
/// text: token.1
pub type Token = (SyntaxKind, String);
pub type TokenStream = Vec<Token>;

/// Deterministic Finite Automaton
///
/// DFA = ( StateSet, InputSet, transition_fn, start, TerminatorSet )
///
/// StateSet = { START, OPERATOR, ZERO, NUM }
/// InputSet = { operator, whitespace, 0, 1-9 }
/// start = START
/// TerminatorSet = { OPERATOR, ZERO, NUM }
///
/// transition_table
/// |              | op  | ws  | 0   | 1-9 |
/// |--------------|-----|-----|-----|-----|
/// | ERROR        | E   | E   | E   | E   |
/// | START        | 2   | 1   | 3   | 4   |
/// | OPERATOR     | 2   | 1   | 3   | 4   |
/// | ZERO         | 2   | 1   | E   | E   |
/// | NUM          | 2   | 1   | 4   | 4   |
///
pub struct DFA {
    code: String,
    token_stream: TokenStream,
}

impl DFA {
    pub fn new(code: String) -> Self {
        DFA {
            code,
            token_stream: Vec::new(),
        }
    }
    pub fn token_stream(&self) -> TokenStream {
        self.token_stream.to_owned()
    }
    pub fn run(&mut self) -> Result<(), String> {
        if self.code.len() == 0 {
            return Err("an empty string was received".to_string());
        }

        // state transition function
        let transition = get_transition();
        // termination state judgument helper
        let is_terminator = get_terminator_judgement();

        let mut idx = 0;
        let mut state = START;
        let mut prev_state = ERROR;

        // cache currently matched characters
        let mut text_cache = String::new();
        while let Some(c) = self.code.chars().nth(idx) {
            // 1. judge which state to transfer to according to the current character
            state = transition(c, state);

            // 2.1 panic at ERROR state
            if state == ERROR {
                return Err(format!(
                    "unexpected token at the {} of the input, current cache: {}",
                    idx, text_cache
                ));
            }
            // 2.2 save the contents of the cache as a token and make it empty
            //     while:
            //          (1). a transition between two termination states occurs
            //          (2). an operator was matched
            else if is_terminator(prev_state) && (state != prev_state || prev_state == OPERATOR) {
                self.push_token(&text_cache);
                text_cache.clear();
            }

            // 3. save the current character to the cache, except for whitespace
            if c != ' ' {
                text_cache.push(c);
            }

            // 4. update the index and previous state
            //    back to the step 1
            idx += 1;
            prev_state = state;
        }
        // 5. the last one token
        self.push_token(&text_cache);

        Ok(())
    }
    fn push_token(&mut self, text: &str) {
        let token = match SyntaxKind::from_operator(text) {
            // is a operator token
            Some(kind) => (kind, text.to_string()),
            // is a number token, and it is possible to merge
            // e.g
            //         source: "1 + -1"
            //   token_stream: [ 1, +, - ]  <- 1
            //
            //   Awesome: [ 1, +, -1 ]
            //       Bad: [ 1, +, -, 1 ]
            None => (NUM, self.try_merge(text.to_string())),
        };
        self.token_stream.push(token);
    }
    fn try_merge(&mut self, mut text: String) -> String {
        let len = self.token_stream.len();

        // e.g
        //   [ 1, +, - ] <- 1
        //        |  |      |
        //        k1 k2   text
        if len >= 2 {
            let (k1, _) = self.token_stream[len - 2];
            let (k2, _) = self.token_stream[len - 1];
            match k1 {
                token!["+"] | token!["-"] | token!["*"] | token!["/"] | token!["("] => match k2 {
                    // "1 + - 1" => [ 1, +, -1 ]
                    token!["-"] => {
                        self.token_stream.pop();
                        text.insert_str(0, "-")
                    }
                    // "1 + + 1" => [ 1, +, 1]
                    token!["+"] => {
                        self.token_stream.pop();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // e.g
        //   [ - ] <- 1
        //     |      |
        //     k1    text
        else if len == 1 {
            let (k1, _) = self.token_stream[len - 1];
            if k1 == token!["+"] || k1 == token!["-"] {
                self.token_stream.pop();
                match k1 {
                    // "- 1" => [ -1 ]
                    token!["-"] => text.insert_str(0, "-"),
                    // "+ 1" => [ 1 ]
                    _ => {}
                }
            }
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::dfa::DFA;
    use crate::lexer::TokenStream;
    use crate::syntax_kind::{NUM, PLUS};

    fn lex(code: &str) -> Result<TokenStream, ()> {
        let mut dfa = DFA::new(code.to_string());
        if dfa.run().is_ok() {
            Ok(dfa.token_stream())
        } else {
            Err(())
        }
    }

    #[test]
    fn basic_test() {
        assert_eq!(vec![(NUM, "123".to_string())], lex("+123").unwrap());
        assert_eq!(vec![(NUM, "-123".to_string())], lex("-123").unwrap());
        assert_eq!(vec![(NUM, "123".to_string())], lex("123").unwrap());

        assert_eq!(
            vec![
                (NUM, "123".to_string()),
                (PLUS, "+".to_string()),
                (NUM, "-456".to_string())
            ],
            lex("123+-456").unwrap()
        );
        assert_eq!(
            vec![
                (NUM, "123".to_string()),
                (PLUS, "+".to_string()),
                (NUM, "-456".to_string())
            ],
            lex("+123+-456").unwrap()
        );
        assert_eq!(
            vec![
                (NUM, "123".to_string()),
                (PLUS, "+".to_string()),
                (NUM, "456".to_string())
            ],
            lex("+123+456").unwrap()
        );
    }

    #[test]
    fn allow_positive_zero_and_negative_zero() {
        assert_eq!(vec![(NUM, "0".to_string())], lex("0").unwrap());
        assert_eq!(vec![(NUM, "-0".to_string())], lex("-0").unwrap());
        assert_eq!(vec![(NUM, "0".to_string())], lex("0").unwrap());
    }

    #[test]
    fn allow_suffix_zero() {
        assert_eq!(vec![(NUM, "100".to_string())], lex("+100").unwrap());
        assert_eq!(vec![(NUM, "-100".to_string())], lex("-100").unwrap());
        assert_eq!(vec![(NUM, "100".to_string())], lex("100").unwrap());
    }

    #[test]
    fn panic_at_prefix_zero() {
        assert!(lex("+001").is_err());
        assert!(lex("-001").is_err());
        assert!(lex("001").is_err());
    }
}
