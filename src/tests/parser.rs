use crate::{
    lexer::lib::Lexer,
    parser::{
        self,
        ast::{Expr, Node, UnaryOp},
    },
};

#[test]
fn it_can_do_something() {
    let input = r#"
        let a = "hello";
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let ast = parser::parse(tokens).unwrap();
    println!("{:?}", ast);
    // let expected = vec![Node::LetDecl {
    //     identifier: String::from("a"),
    //     expression: Expr::UnaryExpr {
    //         op: UnaryOp::Assign,
    //         expr: todo!(),
    //     },
    // }];
    assert_ne!(ast.len(), 0);
}
