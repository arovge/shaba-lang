use crate::{
    lexer,
    parser::{
        self,
        ast::{BinaryOp, Decl, Expr, Node, UnaryOp},
        error::{ExpectedToken, ParserError, ParsingError},
    },
};

fn try_parse_str(input: &str) -> Result<Vec<Node>, ParserError> {
    let tokens = lexer::tokenize(input).unwrap();
    parser::parse(tokens)
}

fn parse_str(input: &str) -> Vec<Node> {
    try_parse_str(input).unwrap()
}

#[test]
#[ignore]
fn parses_unit() {
    let input = "()";
    let result = parse_str(input);
    let expected = vec![Node::Expr(Expr::Unit)];
    assert_eq!(result, expected);
}

#[test]
fn parses_literal_int() {
    let input = "1";
    let result = parse_str(input);
    let expected = vec![Node::Expr(Expr::Int(1))];
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn parses_literal_double() {
    let input = "1.234";
    let result = parse_str(input);
    let expected = vec![Node::Expr(Expr::Double(1.234))];
    assert_eq!(result, expected);
}

#[test]
fn parses_literal_str() {
    let input = "\"eeeoo\"";
    let result = parse_str(input);
    let expected = vec![Node::Expr(Expr::String("eeeoo".to_string()))];
    assert_eq!(result, expected);
}

#[test]
fn parses_literal_bool() {
    let input = r#"
        true
        false
    "#;
    let result = parse_str(input);
    let expected = vec![Node::Expr(Expr::Bool(true)), Node::Expr(Expr::Bool(false))];
    assert_eq!(result, expected);
}

#[test]
fn parses_equality() {
    let input = r#"
        1 == 1
        1 != 21
    "#;
    let result = parse_str(input);
    let expected = vec![
        Node::Expr(Expr::Binary {
            op: BinaryOp::Eq,
            lhs: Box::new(Expr::Int(1)),
            rhs: Box::new(Expr::Int(1)),
        }),
        Node::Expr(Expr::Binary {
            op: BinaryOp::NotEq,
            lhs: Box::new(Expr::Int(1)),
            rhs: Box::new(Expr::Int(21)),
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn parses_cmp() {
    let input = r#"
        11 > 8
        50 >= 50
        5 <= 5
        -3 < 3
    "#;
    let result = parse_str(input);
    let expected = vec![
        Node::Expr(Expr::Binary {
            op: BinaryOp::GreaterThan,
            lhs: Box::new(Expr::Int(11)),
            rhs: Box::new(Expr::Int(8)),
        }),
        Node::Expr(Expr::Binary {
            op: BinaryOp::GreaterThanEq,
            lhs: Box::new(Expr::Int(50)),
            rhs: Box::new(Expr::Int(50)),
        }),
        Node::Expr(Expr::Binary {
            op: BinaryOp::LessThanEq,
            lhs: Box::new(Expr::Int(5)),
            rhs: Box::new(Expr::Int(5)),
        }),
        Node::Expr(Expr::Binary {
            op: BinaryOp::LessThan,
            lhs: Box::new(Expr::Int(-3)),
            rhs: Box::new(Expr::Int(3)),
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn parses_term() {
    let input = r#"
        1 / 2
        3 * 4
    "#;
    let result = parse_str(input);
    let expected = vec![
        Node::Expr(Expr::Binary {
            op: BinaryOp::Divide,
            lhs: Box::new(Expr::Int(1)),
            rhs: Box::new(Expr::Int(2)),
        }),
        Node::Expr(Expr::Binary {
            op: BinaryOp::Multiply,
            lhs: Box::new(Expr::Int(3)),
            rhs: Box::new(Expr::Int(4)),
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn parses_factor() {
    let input = r#"
        1 / 2
        3 * 4
    "#;
    let result = parse_str(input);
    let expected = vec![
        Node::Expr(Expr::Binary {
            op: BinaryOp::Divide,
            lhs: Box::new(Expr::Int(1)),
            rhs: Box::new(Expr::Int(2)),
        }),
        Node::Expr(Expr::Binary {
            op: BinaryOp::Multiply,
            lhs: Box::new(Expr::Int(3)),
            rhs: Box::new(Expr::Int(4)),
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
fn parses_unary() {
    let input = r#"
        -7
        !21
        !!-36
    "#;
    let result = parse_str(input);
    let expected = vec![
        Node::Expr(Expr::Unary {
            op: UnaryOp::Minus,
            expr: Box::new(Expr::Int(7)),
        }),
        Node::Expr(Expr::Unary {
            op: UnaryOp::Negate,
            expr: Box::new(Expr::Int(21)),
        }),
        Node::Expr(Expr::Unary {
            op: UnaryOp::Negate,
            expr: Box::new(Expr::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(Expr::Unary {
                    op: UnaryOp::Minus,
                    expr: Box::new(Expr::Int(36)),
                }),
            }),
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
fn unterminated_grouping() {
    let input = r#"
        (5 > 3
    "#;
    let result = try_parse_str(input).unwrap_err();
    let expected = ParserError {
        errors: vec![ParsingError::ExpectedToken(ExpectedToken::ClosingParen)],
    };
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn parses_unit_let_decl() {
    let input = r#"
        let unit_decl = ()
        let int_decl = 1234
        let str_decl = "hello world"
        let bool_decl = false
    "#;
    let result = parse_str(input);
    let expected = vec![
        Node::Decl(Decl::Let {
            identifier: "unit_decl".to_string(),
            expr: Expr::Unit,
        }),
        Node::Decl(Decl::Let {
            identifier: "int_decl".to_string(),
            expr: Expr::Int(1234),
        }),
        Node::Decl(Decl::Let {
            identifier: "str_decl".to_string(),
            expr: Expr::String("hello world".to_string()),
        }),
        Node::Decl(Decl::Let {
            identifier: "bool_decl".to_string(),
            expr: Expr::Bool(false),
        }),
    ];
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn parses_fn_decl() {
    let input = r#"
        fn some_func() {

        }
    "#;
    let result = parse_str(input);
    let expected = vec![Node::Decl(Decl::Fn {
        identifier: "some_func".to_string(),
    })];
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn decl_unary_expr() {
    let input = r#"
        let a = -5
    "#;
    let result = parse_str(input);
    let expected = vec![Node::Decl(Decl::Let {
        identifier: "a".to_string(),
        expr: Expr::Unary {
            op: UnaryOp::Minus,
            expr: Box::new(Expr::Int(5)),
        },
    })];
    assert_eq!(result, expected);
}
