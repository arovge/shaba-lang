use super::ast::Decl;
use super::scanner::Scanner;
use super::{
    ast::{Cmp, Expr, Node, Op, UnaryOp},
    error::{ExpectedToken, ParserError, ParsingError},
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
        while !self.scanner.is_eof() {
            statements.push(Node::Expr(self.expr()));
        }
        // assert!(self.scanner.is_eof(), "Not at end of tokens");
        if !self.errors.is_empty() {
            return Err(ParserError::new(self.errors));
        }
        Ok(statements)
    }

    // fn statement(&mut self) -> Option<Node> {
    //     // decl | expr ;
    //     self.decl()
    //         .map(Node::Decl)
    //         .or_else(|| self.expr().map(Node::Expr))
    // }

    // fn decl(&mut self) -> Option<Decl> {
    //     // let_decl | fn_decl
    //     self.let_decl().or_else(|| self.fn_decl())
    // }

    // fn fn_decl(&mut self) -> Option<Decl> {
    //     self.scanner
    //         .next_if(|x| x.as_keyword() == Some(Keyword::Fn))?;

    //     let identifier = self.next_if_identifier().expect("Expected identifier");
    //     self.scanner
    //         .next_if(|x| matches!(x.kind(), TokenKind::OpenParen))
    //         .expect("Expected '(' in fn decl");

    //     // TODO: Parse args

    //     self.scanner
    //         .next_if(|x| matches!(x.kind(), TokenKind::CloseParen))
    //         .expect("Expected ')' in fn decl");

    //     // TODO: Parse return type

    //     self.scanner
    //         .next_if(|x| matches!(x.kind(), TokenKind::OpenBrace))
    //         .expect("Expected '{' in fn decl");

    //     // TOOD: Parse fn body

    //     self.scanner
    //         .next_if(|x| matches!(x.kind(), TokenKind::CloseBrace))
    //         .expect("Expected '}' in fn decl");

    //     Some(Decl::Fn { identifier })
    // }

    // fn let_decl(&mut self) -> Option<Decl> {
    //     self.scanner
    //         .next_if(|x| x.as_keyword() == Some(Keyword::Let))?;
    //     let identifier = self.next_if_identifier().expect("Expected identifier");
    //     self.scanner
    //         .next_if(|x| *x.kind() == TokenKind::Eq)
    //         .expect("Expected = in let decl");
    //     let expr = self
    //         .expr()
    //         .expect("Expected expression after `let <ident> = `");
    //     Some(Decl::Let { identifier, expr })
    // }

    // fn expr(&mut self) -> Option<Expr> {
    //     // if_expr | unit_expr | literal | unary_expr | binary_expr ;
    //     self.unit_expr()
    //         .or_else(|| self.cmp_expr())
    //         .or_else(|| self.literal())
    //         .or_else(|| self.unary_expr())
    // }

    // fn unit_expr(&mut self) -> Option<Expr> {
    //     let is_unit_expr = matches!(self.scanner.peek()?.kind(), TokenKind::OpenParen)
    //         && matches!(self.scanner.peek_next()?.kind(), TokenKind::CloseParen);
    //     if is_unit_expr {
    //         self.scanner.next();
    //         self.scanner.next();
    //         Some(Expr::Unit)
    //     } else {
    //         None
    //     }
    // }

    // fn unary_expr(&mut self) -> Option<Expr> {
    //     let op = self.scanner.next_if_map(|x| UnaryOp::from(&x))?;
    //     let expr = self.expr().expect("Expected expr after unary op");
    //     Some(Expr::Unary {
    //         op,
    //         expr: Box::new(expr),
    //     })
    // }

    // fn literal(&mut self) -> Option<Expr> {
    //     self.scanner.next_if_map(|x| x.as_literal_expr())
    // }

    // TODO: everything below is GOOD
    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.cmp();

        while let Some(op) = self.scanner.next_if(|x| {
            matches!(x.kind(), TokenKind::EqEq) || matches!(x.kind(), TokenKind::NotEq)
        }) {
            let op = op.as_binary_op().unwrap();
            let rhs = self.cmp();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    fn cmp(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(op) = self.scanner.next_if(|x| {
            matches!(x.kind(), TokenKind::GreaterThan)
                || matches!(x.kind(), TokenKind::GreaterThanEq)
                || matches!(x.kind(), TokenKind::LessThan)
                || matches!(x.kind(), TokenKind::LessThanEq)
        }) {
            let op = op.as_binary_op().unwrap();
            let rhs = self.term();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(op) = self.scanner.next_if(|x| {
            matches!(x.kind(), TokenKind::Minus) || matches!(x.kind(), TokenKind::Plus)
        }) {
            let op = op.as_binary_op().unwrap();
            let rhs = self.factor();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Some(op) = self.scanner.next_if(|x| {
            matches!(x.kind(), TokenKind::Slash) || matches!(x.kind(), TokenKind::Asterisk)
        }) {
            let op = op.as_binary_op().unwrap();
            let rhs = self.unary();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let op = self.scanner.next_if_map(|x| x.as_unary_op());
        if let Some(op) = op {
            let expr = self.primary();
            return Expr::Unary {
                op,
                expr: Box::new(expr),
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        let literal = self.next_if_literal_expr();
        if literal.is_some() {
            return literal.unwrap();
        }
        let open_paren = self
            .scanner
            .next_if(|x| matches!(x.kind(), TokenKind::OpenParen));
        if open_paren.is_none() {
            panic!();
        }
        let expr = self.expr();
        let closing_paren = self
            .scanner
            .next_if(|x| matches!(x.kind(), TokenKind::CloseParen));
        if closing_paren.is_none() {
            self.errors
                .push(ParsingError::ExpectedToken(ExpectedToken::ClosingParen));
            // panic!();
        }
        expr
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

    fn next_if_op(&mut self) -> Option<Op> {
        self.scanner.next_if_map(|x| x.as_op())
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
