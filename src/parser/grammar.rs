use crate::{
    parser::{
        node::{Node, Node::*},
        parser_combinator::{either, single_token, zero_or_more, Parser},
    },
    syntax_kind::{SyntaxKind, ADD_EXPR, DIV_EXPR, MUL_EXPR, NUM, SUB_EXPR, UNKNOW},
    token,
};

/// Literal -> NUM
pub fn literal() -> impl Parser<'static, Node> {
    single_token(NUM).map(|(_, value)| Literal {
        kind: NUM,
        value: value.parse().unwrap(),
        raw: value,
    })
}

/// Expr -> Term (("+" | "-") Term)*
pub fn expr() -> impl Parser<'static, Node> {
    term().and_then(|left| {
        zero_or_more(
            either(single_token(token!["+"]), single_token(token!["-"]))
                .and_then(|(op, _)| term().map(move |right| (op, right))),
        )
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_expr_node(left.to_owned(), node_list),
            }
        })
    })
}

/// Term -> Factor (("*" | "/") Factor)*
fn term() -> impl Parser<'static, Node> {
    factor().and_then(|left| {
        zero_or_more(
            either(single_token(token!["*"]), single_token(token!["/"]))
                .and_then(|(op, _)| factor().map(move |right| (op, right))),
        )
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_expr_node(left.to_owned(), node_list),
            }
        })
    })
}

/// Factor -> Literal | "(" Expr ")"
fn factor() -> impl Parser<'static, Node> {
    either(
        literal(),
        single_token(token!["("])
            .and_then(|_| expr())
            .and_then(|node| single_token(token![")"]).map(move |_| node.to_owned())),
    )
}

/// build expression node recursively
fn build_expr_node(expr: Node, mut node_list: Vec<(SyntaxKind, Node)>) -> Node {
    match node_list.len() {
        0 => expr,
        _ => {
            let (op, right) = node_list.pop().unwrap();
            Expr {
                kind: match op {
                    token!["+"] => ADD_EXPR,
                    token!["-"] => SUB_EXPR,
                    token!["*"] => MUL_EXPR,
                    token!["/"] => DIV_EXPR,
                    _ => UNKNOW,
                },
                /// the `Top-Down Parsing` can not deal with Left Recursive grammar
                /// however, the basic operations are inherently `Left Associative`
                /// therefore, it must be recursive to the left to simulate left recursion when building nodes
                /// just make the AST grow to the left
                left: Box::new(build_expr_node(expr, node_list)),
                op,
                right: Box::new(right),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex;
    use crate::syntax_kind::{MINUS, PLUS, SLASH, STAR};

    fn get_number() -> (Box<Node>, Box<Node>, Box<Node>, Box<Node>, Box<Node>) {
        let one = Box::new(Literal {
            kind: NUM,
            value: 1,
            raw: "1".to_string(),
        });
        let two = Box::new(Literal {
            kind: NUM,
            value: 2,
            raw: "2".to_string(),
        });
        let three = Box::new(Literal {
            kind: NUM,
            value: 3,
            raw: "3".to_string(),
        });
        let four = Box::new(Literal {
            kind: NUM,
            value: 4,
            raw: "4".to_string(),
        });
        let five = Box::new(Literal {
            kind: NUM,
            value: 5,
            raw: "5".to_string(),
        });
        (one, two, three, four, five)
    }
    fn get_expr() -> (Box<Node>, Box<Node>, Box<Node>) {
        let (one, two, three, four, _) = get_number();
        let one_plus_two = Box::new(Expr {
            kind: ADD_EXPR,
            left: one.clone(),
            op: PLUS,
            right: two.clone(),
        });
        let two_plus_three = Box::new(Expr {
            kind: ADD_EXPR,
            left: two.clone(),
            op: PLUS,
            right: three.clone(),
        });
        let three_plus_four = Box::new(Expr {
            kind: ADD_EXPR,
            left: three.clone(),
            op: PLUS,
            right: four.clone(),
        });
        (one_plus_two, two_plus_three, three_plus_four)
    }

    #[test]
    fn test_literal() {
        let input = lex("1").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Literal {
                    kind: NUM,
                    value: 1,
                    raw: "1".to_string()
                }
            )),
            literal().parse(input)
        );
    }

    #[test]
    fn test_expr() {
        let (one, two, three, _, _) = get_number();
        let (one_plus_two, _, _) = get_expr();

        let input = lex("1 + 2 - 3").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: SUB_EXPR,
                    left: one_plus_two.clone(),
                    op: MINUS,
                    right: three.clone()
                }
            )),
            expr().parse(input)
        );
        let input = lex("1 * 2 / 3").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: DIV_EXPR,
                    left: Box::new(Expr {
                        kind: MUL_EXPR,
                        left: one.clone(),
                        op: STAR,
                        right: two.clone()
                    }),
                    op: SLASH,
                    right: three.clone()
                }
            )),
            expr().parse(input)
        );

        let input = lex("( 1 )").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Literal {
                    kind: NUM,
                    value: 1,
                    raw: "1".to_string()
                }
            )),
            expr().parse(input)
        );
    }

    #[test]
    fn mul_precedence_over_add_and_sub() {
        let (one, two, three, four, _) = get_number();
        let (one_plus_two, _, _) = get_expr();

        let input = lex("1 + 2 * 3").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: ADD_EXPR,
                    left: one.clone(),
                    op: PLUS,
                    right: Box::new(Expr {
                        kind: MUL_EXPR,
                        left: two.clone(),
                        op: STAR,
                        right: three.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        let input = lex("1 + 2 - 3 * 4").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: SUB_EXPR,
                    left: one_plus_two.clone(),
                    op: MINUS,
                    right: Box::new(Expr {
                        kind: MUL_EXPR,
                        left: three.clone(),
                        op: STAR,
                        right: four.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        let input = lex("1 + 2 * 3 * 4").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: ADD_EXPR,
                    left: one.clone(),
                    op: PLUS,
                    right: Box::new(Expr {
                        kind: MUL_EXPR,
                        left: Box::new(Expr {
                            kind: MUL_EXPR,
                            left: two.clone(),
                            op: STAR,
                            right: three.clone()
                        }),
                        op: STAR,
                        right: four.clone()
                    })
                }
            )),
            expr().parse(input)
        );
    }

    #[test]
    fn div_precedence_over_add_and_sub() {
        let (one, two, three, four, _) = get_number();
        let (one_plus_two, _, _) = get_expr();

        let input = lex("1 + 2 / 3").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: ADD_EXPR,
                    left: one.clone(),
                    op: PLUS,
                    right: Box::new(Expr {
                        kind: DIV_EXPR,
                        left: two.clone(),
                        op: SLASH,
                        right: three.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        let input = lex("1 + 2 - 3 / 4").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: SUB_EXPR,
                    left: one_plus_two.clone(),
                    op: MINUS,
                    right: Box::new(Expr {
                        kind: DIV_EXPR,
                        left: three.clone(),
                        op: SLASH,
                        right: four.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        let input = lex("1 + 2 / 3 / 4").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: ADD_EXPR,
                    left: one.clone(),
                    op: PLUS,
                    right: Box::new(Expr {
                        kind: DIV_EXPR,
                        left: Box::new(Expr {
                            kind: DIV_EXPR,
                            left: two.clone(),
                            op: SLASH,
                            right: three.clone()
                        }),
                        op: SLASH,
                        right: four.clone()
                    })
                }
            )),
            expr().parse(input)
        );
    }

    #[test]
    fn paren_expr_have_the_highest_priority() {
        let (one, _, _, _, _) = get_number();
        let (_, two_plus_three, _) = get_expr();

        let input = lex("1 * ( 2 + 3 )").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: MUL_EXPR,
                    left: one.clone(),
                    op: STAR,
                    right: two_plus_three.clone()
                }
            )),
            expr().parse(input)
        );
    }

    #[test]
    fn allow_to_nest_paren() {
        let (one, two, _, _, _) = get_number();
        let (_, _, three_plus_four) = get_expr();

        let input = lex("1 * ( 2 + ( 3 + 4 ) )").unwrap();
        assert_eq!(
            Ok((
                vec![],
                Expr {
                    kind: MUL_EXPR,
                    left: one.clone(),
                    op: STAR,
                    right: Box::new(Expr {
                        kind: ADD_EXPR,
                        left: two.clone(),
                        op: PLUS,
                        right: three_plus_four.clone()
                    })
                }
            )),
            expr().parse(input)
        );
    }
}
