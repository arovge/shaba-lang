use super::{
    ast::{Cmp, Node, Operator},
    error::ParserError,
};
use crate::lexer::token::TokenKind as LexerTokenKind;
use crate::lexer::{
    token::{Keyword, Literal, Token},
    Lexer,
};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ParserError> {
        println!("{:?}", self.tokens);
        let mut statements = Vec::new();
        while self.is_not_at_eof() {
            if let Some(exp) = self.declaration() {
                statements.push(exp);
            }
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Option<Node> {
        if let Some(_keyword) = self.next_if_keyword() {
            if let Some(identifier) = self.next_if_identifier() {
                return Some(Node::VarDecl { identifier });
            }
        }
        let statement = self.statement();
        if statement.is_none() {
            self.synchronize();
            return None;
        }
        statement
    }

    fn statement(&mut self) -> Option<Node> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Node> {
        self.equality()
    }

    // TODO: make sure next
    fn equality(&mut self) -> Option<Node> {
        let mut expression = self.comparison()?;

        let current = self.next_if(|x| matches!(x.kind(), LexerTokenKind::Eq));
        if let Some(eq) = current {
            // TOOD: not equal
            let operator = self.peek_previous().unwrap().as_operator().unwrap();
            let rhs = self.comparison().unwrap();
            expression = Node::BinaryExpression {
                op: operator,
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
            // TOOD: greater equal, less, less equal
            let operator = self.peek_previous().unwrap().as_operator().unwrap();
            let rhs = self.term().unwrap();
            expression = Node::BinaryExpression {
                op: operator,
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

        let current = self.next().unwrap().kind().clone();
        while matches!(current, LexerTokenKind::Asterisk)
            || matches!(current, LexerTokenKind::Slash)
        {
            // TODO: Wtf is an operator at this point
            let operator = self.peek_previous().unwrap().as_operator().unwrap();
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
        return Some(node);
    }

    fn primary(&mut self) -> Option<Node> {
        println!("prim");
        let literal = self.next_if_literal_node();
        if literal.is_some() {
            return literal;
        }
        let exp = self.expression();
        let closing_paren = self.next_if(|x| matches!(x.kind(), LexerTokenKind::CloseParen));
        if closing_paren.is_none() {
            self.errors.push(ParserError::ExpectedToken {
                expected_token: super::error::ExpectedToken::ClosingParen,
            });
        }
        exp
    }

    fn synchronize(&mut self) {
        self.increment_cursor();
        loop {
            let Some(prev) = self.peek_previous() else { return; };
            let prev_kind = prev.kind();
            if matches!(prev_kind, LexerTokenKind::Semicolon) {
                return;
            }
            let Some(current) = self.peek() else { return; };
            let current_kind = current.kind();
            if matches!(current_kind, LexerTokenKind::Keyword(_)) {
                return;
            }
            self.increment_cursor();
        }
    }

    fn peek_previous(&self) -> Option<&Token> {
        self.tokens.get(self.cursor - 1)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.cursor + 1)
    }

    fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.cursor + n)
    }

    fn increment_cursor(&mut self) {
        self.cursor += 1;
    }

    fn next(&mut self) -> Option<Token> {
        self.increment_cursor();
        self.tokens.get(self.cursor - 1)?.clone().into()
    }

    fn next_if(&mut self, condition: impl Fn(&Token) -> bool) -> Option<Token> {
        let next = self.tokens.get(self.cursor)?.clone();

        if !condition(&next) {
            return None;
        }

        self.increment_cursor();

        return Some(next);
    }

    fn next_if_operator(&mut self) -> Option<Operator> {
        self.next_map(|x| x.as_operator())
    }

    fn next_if_literal_node(&mut self) -> Option<Node> {
        self.next_map(|x| x.as_literal_node())
    }

    fn next_if_keyword(&mut self) -> Option<Keyword> {
        self.next_map(|x| x.as_keyword())
    }

    fn next_if_identifier(&mut self) -> Option<String> {
        self.next_map(|x| x.as_identifier())
    }

    fn next_while(&mut self, condition: impl Fn(&Token) -> bool) {
        while let Some(_) = self.next_if(&condition) {}
    }

    fn next_map<T>(&mut self, map: impl Fn(&Token) -> Option<T>) -> Option<T> {
        let next = self.tokens.get(self.cursor)?.clone();
        let result = map(&next)?;
        self.increment_cursor();
        Some(result)
    }

    fn next_if_cmp(&mut self) -> Option<Cmp> {
        self.next_map(|x| x.as_comparison())
    }

    fn is_not_at_eof(&self) -> bool {
        self.tokens.get(self.cursor).is_some()
    }
}