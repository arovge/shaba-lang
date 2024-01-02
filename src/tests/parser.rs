use crate::{
    lexer::{self},
    parser::{
        self,
        ast::{Decl, Expr, Node, UnaryOp},
        error::ParserError,
    },
};

fn parse_str(input: &str) -> Result<Vec<Node>, ParserError> {
    let tokens = lexer::tokenize(input).unwrap();
    parser::parse(tokens)
}

#[test]
fn parses_unit() {
    let input = r#"
        ()
    "#;
    let result = parse_str(input).unwrap();
    let expected = vec![Node::Expr(Expr::Unit)];
    assert_eq!(result, expected);
}

#[test]
fn parses_literals() {
    let input = r#"
        1
        "eeeoo"
        true
    "#;
    let result = parse_str(input).unwrap();
    let expected = vec![
        Node::Expr(Expr::Int(1)),
        // TODO: Doubles not supported rn
        // Node::Expr(Expr::Double(1.2)),
        Node::Expr(Expr::String("eeeoo".to_string())),
        Node::Expr(Expr::Bool(true)),
    ];
    assert_eq!(result, expected);
}

#[test]
fn parses_unit_let_decl() {
    let input = r#"
        let unit_decl = ()
        let int_decl = 1234
        let str_decl = "hello world"
        let bool_decl = false
    "#;
    let result = parse_str(input).unwrap();
    let expected = vec![
        Node::Decl(Decl::Let {
            identifier: "unit_decl".to_string(),
            expression: Expr::Unit,
        }),
        Node::Decl(Decl::Let {
            identifier: "int_decl".to_string(),
            expression: Expr::Int(1234),
        }),
        Node::Decl(Decl::Let {
            identifier: "str_decl".to_string(),
            expression: Expr::String("hello world".to_string()),
        }),
        Node::Decl(Decl::Let {
            identifier: "bool_decl".to_string(),
            expression: Expr::Bool(false),
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
fn parses_fn_decl() {
    let input = r#"
        fn some_func() {

        }
    "#;
    let result = parse_str(input).unwrap();
    let expected = vec![Node::Decl(Decl::Fn {
        identifier: "some_func".to_string(),
    })];
    assert_eq!(result, expected);
}

#[test]
fn parses_unary_expr() {
    let input = r#"
        let a = -5
    "#;
    let result = parse_str(input).unwrap();
    let expected = vec![Node::Decl(Decl::Let {
        identifier: "a".to_string(),
        expression: Expr::Unary {
            op: UnaryOp::Minus,
            expr: Box::new(Expr::Int(5)),
        },
    })];
    assert_eq!(result, expected);
}
