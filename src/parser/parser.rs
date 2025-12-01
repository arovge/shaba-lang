use super::scanner::Scanner;
use super::{
    ast::{Expr, Node},
    error::{ExpectedToken, ParserError, ParsingError},
};
use crate::lexer::token::{Token, TokenKind};

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
        if !self.errors.is_empty() {
            return Err(ParserError {
                errors: self.errors,
            });
        }
        Ok(statements)
    }

    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.cmp();

        while let Some(op) = self.next_if_map(|x| x.as_equality_op()) {
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

        while let Some(op) = self.next_if_map(|x| x.as_cmp_op()) {
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

        while let Some(op) = self.next_if_map(|x| x.as_term_op()) {
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

        while let Some(op) = self.next_if_map(|x| x.as_factor_op()) {
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
        let op = self.next_if_map(|x| x.as_unary_op());
        if let Some(op) = op {
            let expr = self.unary();
            return Expr::Unary {
                op,
                expr: Box::new(expr),
            };
        }
        return self.literal();
    }

    fn literal(&mut self) -> Expr {
        let literal = self.next_if_map(|x| x.as_literal());
        if let Some(literal) = literal {
            return literal;
        }
        if self.next_token(TokenKind::OpenParen) {
            let expr = self.expr();
            if !self.next_token(TokenKind::CloseParen) {
                self.errors
                    .push(ParsingError::ExpectedToken(ExpectedToken::ClosingParen));
            }
            return expr;
        }

        panic!("no literal to parse");
    }

    fn next_token(&mut self, kind: TokenKind) -> bool {
        self.next_if(|token| token.kind() == &kind).is_some()
    }

    fn next_if(&mut self, cond: impl Fn(Token) -> bool) -> Option<Token> {
        self.scanner.next_if(cond)
    }

    fn next_if_map<T>(&mut self, cond: impl Fn(Token) -> Option<T>) -> Option<T> {
        self.scanner.next_if_map(cond)
    }
}
