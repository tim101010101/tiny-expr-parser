use crate::syntax_kind::SyntaxKind;

/// enumerate the structure of all ast nodes
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Node {
    Literal {
        kind: SyntaxKind,
        value: i32,
        raw: String,
    },

    Expr {
        kind: SyntaxKind,
        left: Box<Node>,
        op: SyntaxKind,
        right: Box<Node>,
    },
}
