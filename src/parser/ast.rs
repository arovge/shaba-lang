use crate::lexer::token::{Keyword, Literal, Token, TokenKind};

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Negate,
    Slash,
    Asterisk,
    Cmp(Cmp),
}

#[derive(Debug, PartialEq)]
pub enum Cmp {
    LessThan,
    LessThanEq,
    Eq,
    GreaterThan,
    GreaterThanEq,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Int(i32),
    Double(f32),
    String(String),
    Bool(bool),
    Expr(Expr),
    Decl(Decl),
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

#[derive(Debug, PartialEq)]
pub enum Decl {
    Let {
        identifier: String,
        expression: Expr,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Unit,
    UnaryExpr {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Minus,
    Negate,
    Assign,
}

impl UnaryOp {
    pub fn from(token: &Token) -> Option<UnaryOp> {
        match token.kind() {
            TokenKind::Minus => UnaryOp::Minus.into(),
            TokenKind::Negate => UnaryOp::Negate.into(),
            _ => None,
        }
    }
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
            Literal::Int(n) => Node::Int(*n),
            Literal::Double(n) => Node::Double(*n),
            Literal::Bool(n) => Node::Bool(*n),
            Literal::String(n) => Node::String(n.clone()),
        };
        Some(node)
    }

    pub fn as_operator(&self) -> Option<Operator> {
        match self.kind() {
            TokenKind::Plus => Some(Operator::Plus),
            TokenKind::Minus => Some(Operator::Minus),
            TokenKind::Negate => Some(Operator::Negate),
            TokenKind::Slash => Some(Operator::Slash),
            TokenKind::Asterisk => Some(Operator::Asterisk),
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
