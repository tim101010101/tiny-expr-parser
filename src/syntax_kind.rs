#[derive(Debug, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub struct SyntaxKind(pub u16);

// node
pub const NUM: SyntaxKind = SyntaxKind(5);
pub const ADD_EXPR: SyntaxKind = SyntaxKind(6);
pub const SUB_EXPR: SyntaxKind = SyntaxKind(7);
pub const MUL_EXPR: SyntaxKind = SyntaxKind(8);
pub const DIV_EXPR: SyntaxKind = SyntaxKind(9);

// token
pub const OPEN_PAREN: SyntaxKind = SyntaxKind(100);
pub const CLOSE_PAREN: SyntaxKind = SyntaxKind(101);
pub const PLUS: SyntaxKind = SyntaxKind(102);
pub const MINUS: SyntaxKind = SyntaxKind(103);
pub const STAR: SyntaxKind = SyntaxKind(104);
pub const SLASH: SyntaxKind = SyntaxKind(105);

// other
pub const UNKNOW: SyntaxKind = SyntaxKind(65534);

impl SyntaxKind {
    /// try to get a operator kind(`SyntaxKind`) from a string reference(`&str`)
    pub fn from_operator(str: &str) -> Option<SyntaxKind> {
        let op = match str {
            "(" => OPEN_PAREN,
            ")" => CLOSE_PAREN,
            "+" => PLUS,
            "-" => MINUS,
            "*" => STAR,
            "/" => SLASH,
            _ => return None,
        };
        Some(op)
    }
    /// get the priority of the operator, and the higner priority will get a bigger value
    pub fn get_op_priority(str: &str) -> usize {
        match str {
            "*" | "/" => 2,
            "+" | "-" => 1,
            _ => usize::MAX,
        }
    }
    /// tranform a kind value(`SyntaxKind`) into a string reference(`&str`)
    pub fn into_str<'a>(self) -> &'a str {
        match self {
            OPEN_PAREN => "(",
            CLOSE_PAREN => ")",
            PLUS => "+",
            MINUS => "-",
            STAR => "*",
            SLASH => "/",
            _ => "unknow",
        }
    }
}

#[macro_export]
macro_rules! token {
    ["("] => { $crate::syntax_kind::OPEN_PAREN };
    [")"] => { $crate::syntax_kind::CLOSE_PAREN };
    ["+"] => { $crate::syntax_kind::PLUS };
    ["-"] => { $crate::syntax_kind::MINUS };
    ["*"] => { $crate::syntax_kind::STAR };
    ["/"] => { $crate::syntax_kind::SLASH };
}

#[cfg(test)]
mod tests {
    use crate::syntax_kind::*;

    #[test]
    fn test_from_operator() {
        assert!(SyntaxKind::from_operator("(").is_some());
        assert!(SyntaxKind::from_operator(")").is_some());
        assert!(SyntaxKind::from_operator("+").is_some());
        assert!(SyntaxKind::from_operator("-").is_some());
        assert!(SyntaxKind::from_operator("*").is_some());
        assert!(SyntaxKind::from_operator("/").is_some());
    }

    #[test]
    fn test_marco() {
        assert_eq!(token!["("], OPEN_PAREN);
        assert_eq!(token![")"], CLOSE_PAREN);
        assert_eq!(token!["+"], PLUS);
        assert_eq!(token!["-"], MINUS);
        assert_eq!(token!["*"], STAR);
        assert_eq!(token!["/"], SLASH);
    }
}
