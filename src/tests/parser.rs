use crate::{
    lexer::{self},
    parser::{
        self,
        ast::{Decl, Expr, Node},
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
fn parses_unit_let_decl() {
    let input = r#"
        let shaba = ()
    "#;
    let result = parse_str(input).unwrap();
    let expected = vec![Node::Decl(Decl::Let {
        identifier: "shaba".to_string(),
        expression: Expr::Unit,
    })];
    assert_eq!(result, expected);
}
