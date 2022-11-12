use crate::parser::Node;
use crate::traversal::visitor::Visitor;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Executor
    }
    pub fn eval(&mut self, node: &Node) -> i32 {
        self.visit(node)
    }
}

impl Visitor<i32> for Executor {
    /// return the actual value of the node directly
    fn visit_num(&mut self, value: i32, _: &str) -> i32 {
        value
    }

    /// operate the expression according to the operator
    /// must visit the left node first
    ///
    /// e.g
    ///
    ///            1 + 2 + 3
    ///
    ///	              +
    ///	            /   \
    ///	           +     3
    ///	          / \
    ///          1   2
    ///
    fn visit_expr(&mut self, left: &Node, op: &str, right: &Node) -> i32 {
        let left = self.visit(left);
        let right = self.visit(right);
        match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,

            _ => panic!("unexpected operator: {}", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Node;
    use crate::traversal::eval::Executor;
    use crate::{lex, syntax};

    fn get_node(code: &str) -> Node {
        syntax(lex(code).unwrap()).unwrap()
    }

    #[test]
    fn smoke() {
        let mut e = Executor::new();

        assert_eq!(3, e.eval(&get_node("2 + 1")));
        assert_eq!(1, e.eval(&get_node("2 - 1")));
        assert_eq!(2, e.eval(&get_node("2 * 1")));
        assert_eq!(2, e.eval(&get_node("2 / 1")));

        assert_eq!(7, e.eval(&get_node("1 + 2 * 3")));
        assert_eq!(5, e.eval(&get_node("1 * ( 2 + 3 )")));
        assert_eq!(14, e.eval(&get_node("1 * ( 2 * ( 3 + 4 ))")));
    }
}
