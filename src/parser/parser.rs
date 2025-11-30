use super::scanner::Scanner;
use super::{
    ast::{Expr, Node},
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
            let expr = self.literal();
            return Expr::Unary {
                op,
                expr: Box::new(expr),
            };
        }
        return self.literal();
    }

    fn literal(&mut self) -> Expr {
        let literal = self.next_if_map(|x| x.as_literal());
        if literal.is_some() {
            return literal.unwrap();
        }
        let open_paren = self.next_if(|x| matches!(x.kind(), TokenKind::OpenParen));
        if open_paren.is_none() {
            panic!();
        }
        let expr = self.expr();
        let closing_paren = self.next_if(|x| matches!(x.kind(), TokenKind::CloseParen));
        if closing_paren.is_none() {
            self.errors
                .push(ParsingError::ExpectedToken(ExpectedToken::ClosingParen));
            // panic!();
        }
        expr
    }

    fn next_if(&mut self, cond: impl Fn(Token) -> bool) -> Option<Token> {
        self.scanner.next_if(cond)
    }

    fn next_if_map<T>(&mut self, cond: impl Fn(Token) -> Option<T>) -> Option<T> {
        self.scanner.next_if_map(cond)
    }

    fn next_if_keyword(&mut self) -> Option<Keyword> {
        self.scanner.next_if_map(|x| x.as_keyword())
    }

    fn next_if_identifier(&mut self) -> Option<String> {
        self.scanner.next_if_map(|x| x.as_identifier())
    }
}
