use crate::parser::Node;
use crate::syntax_kind::SyntaxKind;
use crate::traversal::visitor::Visitor;

pub struct Formatter {
    output: String,
}

impl Formatter {
    pub fn new() -> Self {
        Formatter {
            output: String::new(),
        }
    }
    pub fn format(&mut self, node: &Node) -> &str {
        self.visit(node);
        self.output.as_str()
    }
}

impl Formatter {
    fn push(&mut self, str: &str) {
        self.output.push_str(str)
    }
    /// wrap a expression in parenthesis
    fn push_paren_expr(&mut self, node: &Node) {
        self.push("(");
        self.visit(node);
        self.push(")");
    }
    fn ws(&mut self) {
        self.push(" ");
    }
}

impl Visitor<()> for Formatter {
    fn visit_num(&mut self, _: i32, raw: &str) {
        self.push(raw)
    }

    fn visit_expr(&mut self, left: &Node, op: &str, right: &Node) {
        self.visit(left);
        self.ws();
        self.push(op);
        self.ws();

        // wrap a expression in a parenthsis
        // while:
        //    (1). the current expression has a higher priority than the right expression
        //    (2). current expression is a negative number
        match right {
            Node::Expr { op: next_op, .. }
                if SyntaxKind::get_op_priority(op)
                    > SyntaxKind::get_op_priority(next_op.into_str()) =>
            {
                self.push_paren_expr(right);
            }
            Node::Literal { raw, .. } if raw.chars().nth(0) == Some('-') => {
                self.push_paren_expr(right);
            }
            _ => {
                self.visit(right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Node;
    use crate::traversal::format::Formatter;
    use crate::{lex, syntax};

    fn get_node(code: &str) -> Node {
        syntax(lex(code).unwrap()).unwrap()
    }

    #[test]
    fn smoke() {
        let mut f = Formatter::new();
        assert_eq!("1 * (2 + 3)", f.format(&get_node("1*(2+3)")));

        let mut f = Formatter::new();
        assert_eq!("1 * 2 + 3", f.format(&get_node("1*2+3")));

        let mut f = Formatter::new();
        assert_eq!("1 * (2 + 3) * 4", f.format(&get_node("1*(2+3)*4")));

        let mut f = Formatter::new();
        assert_eq!("1 * 2 * (3 + 4)", f.format(&get_node("1* ( 2 * ( 3 + 4))")));
    }

    #[test]
    fn allow_negative() {
        let mut f = Formatter::new();
        assert_eq!("1 + (-1)", f.format(&get_node("1+(-1)")));

        let mut f = Formatter::new();
        assert_eq!("-1 + 1", f.format(&get_node("-1+1")));
    }
}
