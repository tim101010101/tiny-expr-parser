use crate::{
    parser::Node,
    traversal::{eval::Executor, format::Formatter},
};

mod eval;
mod format;
mod visitor;

/// execute a expression expressed in AST and return its result
pub fn eval(root: &Node) -> i32 {
    Executor::new().eval(root)
}

/// format a expression expressed in AST
pub fn format(root: &Node) -> String {
    Formatter::new().format(root).to_string()
}
