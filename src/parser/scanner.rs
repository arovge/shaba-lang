use crate::lexer::token::Token;

pub struct Scanner {
    items: Vec<Token>,
    index: usize,
}

impl Scanner {
    pub fn new(items: Vec<Token>) -> Self {
        Self { items, index: 0 }
    }

    pub fn is_eof(&self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&self) -> Option<Token> {
        let item = self.items.get(self.index)?;
        Some(item.clone())
    }

    pub fn peek_next(&self) -> Option<Token> {
        let item = self.items.get(self.index + 1)?;
        Some(item.clone())
    }

    pub fn next_if(&mut self, condition: impl Fn(Token) -> bool) -> Option<Token> {
        self.next_if_map(|x| if condition(x.clone()) { Some(x) } else { None })
    }

    pub fn next_if_map<U>(&mut self, map: impl Fn(Token) -> Option<U>) -> Option<U> {
        let next = self.peek()?;
        let result = map(next);
        if result.is_some() {
            self.increment_cursor();
        }
        result
    }

    pub fn increment_cursor(&mut self) {
        self.index += 1;
    }
}
