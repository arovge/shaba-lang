use crate::lexer::token::{Keyword, Literal, Token, TokenKind};

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Negate,
}

#[derive(Debug)]
pub enum Cmp {
    LessThan,
    LessThanEq,
    Eq,
    GreaterThan,
    GreaterThanEq,
}

#[derive(Debug)]
pub enum Node {
    Integer(i32),
    FloatingPoint(f32),
    String(String),
    Bool(bool),
    VarDecl {
        identifier: String,
    },
    Identifier(String),
    UnaryExpression {
        op: Operator,
        term: Box<Node>,
    },
    BinaryExpression {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Token {
    pub fn as_comparison(&self) -> Option<Cmp> {
        match self.kind() {
            TokenKind::LessThan => Cmp::LessThan.into(),
            TokenKind::LessThanEq => Cmp::LessThanEq.into(),
            TokenKind::Eq => Cmp::Eq.into(),
            TokenKind::GreaterThan => Cmp::GreaterThan.into(),
            TokenKind::GreaterThanEq => Cmp::GreaterThanEq.into(),
            _ => None,
        }
    }

    pub fn as_literal_node(&self) -> Option<Node> {
        let TokenKind::Literal(literal) = self.kind() else {
            return None;
        };
        let node = match literal {
            Literal::Integer(n) => Node::Integer(*n),
            Literal::FloatingPoint(n) => Node::FloatingPoint(*n),
            Literal::Bool(n) => Node::Bool(*n),
            Literal::String(n) => Node::String(n.clone()),
        };
        Some(node)
    }

    pub fn as_operator(&self) -> Option<Operator> {
        match self.kind() {
            TokenKind::Plus => Some(Operator::Plus),
            TokenKind::Minus => Some(Operator::Minus),
            TokenKind::ExclaimationPoint => Some(Operator::Negate),
            _ => None,
        }
    }

    pub fn as_keyword(&self) -> Option<Keyword> {
        let TokenKind::Keyword(keyword) = self.kind() else {
            return None;
        };
        Some(keyword.clone())
    }

    pub fn as_identifier(&self) -> Option<String> {
        let TokenKind::Identifier(id) = self.kind() else {
            return None;
        };
        Some(id.clone())
    }
}
