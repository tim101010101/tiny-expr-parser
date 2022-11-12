use crate::parser::Node;

/// Visitor trait
///
/// get a `Threaded AST` with `Visitor Mode`
pub(crate) trait Visitor<T> {
    /// data distribution of different node types is implemented by default
    fn visit(&mut self, node: &Node) -> T {
        match node {
            Node::Literal { value, raw, .. } => self.visit_num(*value, raw),
            Node::Expr {
                left, op, right, ..
            } => self.visit_expr(left, op.into_str(), right),
        }
    }

    /// customize the behavior of accessing Literal nodes
    fn visit_num(&mut self, value: i32, raw: &str) -> T;
    /// customize the behavior of accessing Expression nodes
    fn visit_expr(&mut self, left: &Node, op: &str, right: &Node) -> T;
}
