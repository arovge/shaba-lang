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

impl From<Expr> for Node {
    fn from(expr: Expr) -> Self {
        Node::Expr(expr)
    }
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    Let { identifier: String, expr: Expr },
    Fn { identifier: String },
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Unit,
    Int(i32),
    Double(f32),
    String(String),
    Bool(bool),
    Nil,
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Divide,
    Multiply,
    Minus,
    Plus,
    GreaterThan,
    GreaterThanEq,
    LessThan,
    LessThanEq,
    Eq,
    NotEq,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Minus,
    Negate,
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
    pub fn as_literal(&self) -> Option<Expr> {
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

    pub fn as_unary_op(&self) -> Option<UnaryOp> {
        match self.kind() {
            TokenKind::Minus => Some(UnaryOp::Minus),
            TokenKind::Negate => Some(UnaryOp::Negate),
            _ => None,
        }
    }

    pub fn as_equality_op(&self) -> Option<BinaryOp> {
        match self.kind() {
            TokenKind::EqEq => Some(BinaryOp::Eq),
            TokenKind::NotEq => Some(BinaryOp::NotEq),
            _ => None,
        }
    }

    pub fn as_cmp_op(&self) -> Option<BinaryOp> {
        match self.kind() {
            TokenKind::GreaterThan => Some(BinaryOp::GreaterThan),
            TokenKind::GreaterThanEq => Some(BinaryOp::GreaterThanEq),
            TokenKind::LessThan => Some(BinaryOp::LessThan),
            TokenKind::LessThanEq => Some(BinaryOp::LessThanEq),
            _ => None,
        }
    }

    pub fn as_term_op(&self) -> Option<BinaryOp> {
        match self.kind() {
            TokenKind::Plus => Some(BinaryOp::Plus),
            TokenKind::Minus => Some(BinaryOp::Minus),
            _ => None,
        }
    }

    pub fn as_factor_op(&self) -> Option<BinaryOp> {
        match self.kind() {
            TokenKind::Slash => Some(BinaryOp::Divide),
            TokenKind::Asterisk => Some(BinaryOp::Multiply),
            _ => None,
        }
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
