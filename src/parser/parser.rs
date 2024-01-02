use super::ast::Decl;
use super::error::ExpectedToken;
use super::scanner::Scanner;
use super::{
    ast::{Cmp, Expr, Node, Operator, UnaryOp},
    error::{ParserError, ParsingError},
};
use crate::lexer::token::{Keyword, Token, TokenKind};

pub struct Parser {
    scanner: Scanner,
    errors: Vec<ParsingError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let scanner = Scanner::new(tokens);
        Self {
            scanner,
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Vec<Node>, ParserError> {
        let mut statements = Vec::new();
        while let Some(exp) = self.statement() {
            statements.push(exp);
        }
        if !self.scanner.is_eof() {
            println!("{:?}", self.scanner.peek());
        }
        assert!(self.scanner.is_eof(), "Not at end of tokens");
        if !self.errors.is_empty() {
            return Err(ParserError::new(
                self.errors
            ));
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Option<Node> {
        // decl | expr ;
        self.decl()
            .map(Node::Decl)
            .or_else(|| self.expr().map(Node::Expr))
    }

    fn decl(&mut self) -> Option<Decl> {
        // let_decl | fn_decl
        self.let_decl().or_else(|| self.fn_decl())
    }

    fn fn_decl(&mut self) -> Option<Decl> {
        self.scanner
            .next_if(|x| x.as_keyword() == Some(Keyword::Fn))?;

        let identifier = self.next_if_identifier().expect("Expected identifier");
        self.scanner
            .next_if(|x| matches!(x.kind(), TokenKind::OpenParen))
            .expect("Expected '(' in fn decl");

        // TODO: Parse args

        self.scanner
            .next_if(|x| matches!(x.kind(), TokenKind::CloseParen))
            .expect("Expected ')' in fn decl");

        // TODO: Parse return type

        self.scanner
            .next_if(|x| matches!(x.kind(), TokenKind::OpenBrace))
            .expect("Expected '{' in fn decl");

        // TOOD: Parse fn body

        self.scanner
            .next_if(|x| matches!(x.kind(), TokenKind::CloseBrace))
            .expect("Expected '}' in fn decl");

        Some(Decl::Fn { identifier })
    }

    fn let_decl(&mut self) -> Option<Decl> {
        self.scanner
            .next_if(|x| x.as_keyword() == Some(Keyword::Let))?;
        let identifier = self.next_if_identifier().expect("Expected identifier");
        self.scanner
            .next_if(|x| *x.kind() == TokenKind::Eq)
            .expect("Expected = in let decl");
        let expression = self
            .expr()
            .expect("Expected expression after `let <ident> = `");
        Some(Decl::Let {
            identifier,
            expression,
        })
    }

    fn expr(&mut self) -> Option<Expr> {
        // if_expr | unit_expr | literal | unary_expr | binary_expr ;
        self.unit_expr()
            .or_else(|| self.literal())
    }

    fn unit_expr(&mut self) -> Option<Expr> {
        let is_unit_expr = matches!(self.scanner.peek()?.kind(), TokenKind::OpenParen)
            && matches!(self.scanner.peek_next()?.kind(), TokenKind::CloseParen);
        if is_unit_expr {
            self.scanner.next();
            self.scanner.next();
            Some(Expr::Unit)
        } else {
            None
        }
    }

    fn literal(&mut self) -> Option<Expr> {
        self.scanner.next_if_map(|x| x.as_literal_expr())
    }

    // TODO: everything below is BAD

    fn expr2(&mut self) -> Option<Expr> {
        self.unary_expr2()
        // TODO: More expr types
    }

    fn if_expr() -> Option<Node> {
        None
    }

    fn unary_expr2(&mut self) -> Option<Expr> {
        let op = self.scanner.next_if_map(|x| UnaryOp::from(&x))?;
        let expr = self.expr2().expect("Expected expr after unary op");
        Expr::UnaryExpr {
            op,
            expr: Box::new(expr),
        }
        .into()
    }

    fn declaration(&mut self) -> Option<Node> {
        if let Some(_keyword) = self.next_if_keyword() {
            if let Some(identifier) = self.next_if_identifier() {
                // return Some(Node::VarDecl { identifier });
            }
        }
        let statement = self.statement();
        if statement.is_none() {
            self.synchronize();
            return None;
        }
        statement
    }

    fn old_statement(&mut self) -> Option<Node> {
        self.expression()
    }

    fn var_decl(&mut self) -> Option<Node> {
        None
    }

    fn expression(&mut self) -> Option<Node> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Node> {
        let mut expression = self.comparison()?;

        if let Some(cmp) = self.scanner.next_if_map(|t| t.as_comparison()) {
            let rhs = self.comparison().unwrap();
            expression = Node::BinaryExpression {
                op: Operator::Cmp(cmp),
                lhs: Box::new(expression),
                rhs: Box::new(rhs),
            };
        }

        Some(expression)
    }

    // TODO: make sure next
    fn comparison(&mut self) -> Option<Node> {
        let mut expression = self.term()?;

        while let Some(cmp) = self.next_if_cmp() {
            let rhs = self.term().unwrap();
            expression = Node::BinaryExpression {
                op: Operator::Cmp(cmp),
                lhs: Box::new(expression),
                rhs: Box::new(rhs),
            };
        }

        Some(expression)
    }

    fn term(&mut self) -> Option<Node> {
        let mut expression = self.factor()?;

        while let Some(operator) = self.next_if_operator() {
            let rhs = self.factor().unwrap();
            expression = Node::BinaryExpression {
                op: operator,
                lhs: Box::new(expression),
                rhs: Box::new(rhs),
            };
        }

        Some(expression)
    }

    fn factor(&mut self) -> Option<Node> {
        let mut expression = self.unary_expression()?;

        while let Some(operator) = self.next_if_operator() {
            let rhs = self.unary_expression().unwrap();
            expression = Node::BinaryExpression {
                op: operator,
                lhs: Box::new(expression),
                rhs: Box::new(rhs),
            };
        }

        Some(expression)
    }

    fn binary_expression(&mut self) -> Option<Node> {
        let lhs = self.primary()?;
        // let operator = self.c
        Some(lhs)
    }

    fn unary_expression(&mut self) -> Option<Node> {
        let Some(operator) = self.next_if_operator() else {
            return self.primary();
        };
        let term = self.unary_expression()?;
        let node = Node::UnaryExpression {
            op: operator,
            term: Box::new(term),
        };
        Some(node)
    }

    fn primary(&mut self) -> Option<Node> {
        // let literal = self.next_if_literal_node();
        // if literal.is_some() {
        //     return literal;
        // }
        // let exp = self.expression();
        // let closing_paren = self
        //     .scanner
        //     .next_if(|x| matches!(x.kind(), TokenKind::CloseParen));
        // if closing_paren.is_none() {
        //     self.errors
        //         .push(ParsingError::ExpectedToken(ExpectedToken::ClosingParen));
        // }
        // exp
        None
    }

    fn synchronize(&mut self) {
        self.scanner.increment_cursor();
        loop {
            let Some(prev) = self.scanner.peek_prev() else {
                break;
            };
            let prev_kind = prev.kind();
            if matches!(prev_kind, TokenKind::Semicolon) {
                break;
            }
            let Some(current) = self.scanner.peek() else {
                break;
            };
            let current_kind = current.kind();
            if matches!(current_kind, TokenKind::Keyword(_)) {
                break;
            }
            self.scanner.increment_cursor();
        }
    }

    // fn peek_prev(&self) -> Option<&Token> {
    //     self.tokens.get(self.cursor - 1)
    // }

    // fn peek(&self) -> Option<&Token> {
    //     self.tokens.get(self.cursor)
    // }

    // fn peek_next(&self) -> Option<&Token> {
    //     self.tokens.get(self.cursor + 1)
    // }

    // fn peek_n(&self, n: usize) -> Option<&Token> {
    //     self.tokens.get(self.cursor + n)
    // }

    // fn increment_cursor(&mut self) {
    //     self.cursor += 1;
    // }

    // fn next(&mut self) -> Option<Token> {
    //     self.increment_cursor();
    //     self.peek_prev()?.clone().into()
    // }

    // fn next_if(&mut self, condition: impl Fn(&Token) -> bool) -> Option<Token> {
    //     self.next_map(|x| if condition(x) { Some(x.clone()) } else { None })
    // }

    // fn next_map<T>(&mut self, map: impl Fn(&Token) -> Option<T>) -> Option<T> {
    //     let next = self.tokens.get(self.cursor)?.clone();
    //     let result = map(&next)?;
    //     self.increment_cursor();
    //     Some(result)
    // }

    fn next_if_operator(&mut self) -> Option<Operator> {
        self.scanner.next_if_map(|x| x.as_operator())
    }

    fn next_if_literal_expr(&mut self) -> Option<Expr> {
        self.scanner.next_if_map(|x| x.as_literal_expr())
    }

    fn next_if_keyword(&mut self) -> Option<Keyword> {
        self.scanner.next_if_map(|x| x.as_keyword())
    }

    fn next_if_identifier(&mut self) -> Option<String> {
        self.scanner.next_if_map(|x| x.as_identifier())
    }

    // fn next_while(&mut self, condition: impl Fn(&Token) -> bool) {
    //     loop {
    //         if self.next_if(&condition).is_none() {
    //             break;
    //         }
    //     }
    // }

    fn next_if_cmp(&mut self) -> Option<Cmp> {
        self.scanner.next_if_map(|x| x.as_comparison())
    }

    // fn is_at_end(&self) -> bool {
    //     self.peek().is_none()
    // }
}
