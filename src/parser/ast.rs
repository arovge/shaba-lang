use crate::lexer::token::{Keyword, Literal, Token, TokenKind};

#[derive(Debug, PartialEq)]
pub enum Op {
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
    Expr(Expr),
    Decl(Decl),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    Let {
        identifier: String,
        expression: Expr,
    },
    Fn {
        identifier: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Unit,
    Int(i32),
    Double(f32),
    String(String),
    Bool(bool),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
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

    pub fn as_literal_expr(&self) -> Option<Expr> {
        let TokenKind::Literal(literal) = self.kind() else {
            return None;
        };
        let node = match literal {
            Literal::Int(n) => Expr::Int(*n),
            Literal::Double(n) => Expr::Double(*n),
            Literal::Bool(n) => Expr::Bool(*n),
            Literal::String(n) => Expr::String(n.clone()),
        };
        Some(node)
    }

    pub fn as_op(&self) -> Option<Op> {
        match self.kind() {
            TokenKind::Plus => Some(Op::Plus),
            TokenKind::Minus => Some(Op::Minus),
            TokenKind::Negate => Some(Op::Negate),
            TokenKind::Slash => Some(Op::Slash),
            TokenKind::Asterisk => Some(Op::Asterisk),
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
