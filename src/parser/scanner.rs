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

    pub fn take_while(&mut self, condition: impl Fn(Token) -> bool) -> Option<Vec<Token>> {
        let first_item = self.next_if(&condition)?;
        let mut taken_items = vec![first_item];

        while let Some(item) = self.next_if(&condition) {
            taken_items.push(item);
        }

        Some(taken_items)
    }

    pub fn advance_while(&mut self, condition: impl Fn(Token) -> bool) {
        loop {
            if self.next_if(&condition).is_none() {
                break;
            }
        }
    }

    // pub fn advance_past_next(&mut self, ch: T) {
    //     self.advance_while(|c| c != ch);
    //     if self.peek().is_some() {
    //         self.next();
    //     }
    // }

    pub fn peek_prev(&self) -> Option<Token> {
        let item = self.items.get(self.index - 1)?;
        Some(item.clone())
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

    pub fn next(&mut self) -> Option<Token> {
        let item = self.peek()?;

        self.increment_cursor();

        Some(item.clone())
    }

    pub fn increment_cursor(&mut self) {
        self.index += 1;
    }

    // fn is_next_char_whitespace(&self) -> bool {
    //     let Some(ch) = self.peek() else {
    //         return false;
    //     };
    //     ch.is_ascii_whitespace()
    // }

    // fn advance_to_next_line(&mut self) {
    //     self.advance_past_next('\n');
    // }
}
