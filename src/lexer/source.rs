use super::token::SourceLocation;

pub struct Source {
    chars: Vec<char>,
    cursor: usize,
    line_position: usize,
    column_position: usize,
}

impl Source {
    pub fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Self {
            chars,
            cursor: 0,
            line_position: 1,
            column_position: 1,
        }
    }

    pub fn position(&self) -> SourceLocation {
        SourceLocation::new(self.line_position, self.column_position)
    }

    pub fn advance_to_next_token(&mut self) {
        while self.is_at_start_of_comment() || self.is_next_char_whitespace() {
            if self.is_at_start_of_comment() {
                self.advance_to_next_line();
            }
            self.advance_while(|ch| ch.is_ascii_whitespace());
        }
    }

    pub fn take_while(&mut self, condition: impl Fn(char) -> bool) -> Option<String> {
        let first_ch = self.next_if(&condition)?;
        let mut chars: Vec<char> = vec![first_ch];

        while let Some(ch) = self.next_if(&condition) {
            chars.push(ch);
        }

        let str = String::from_iter(chars);
        Some(str)
    }

    pub fn advance_while(&mut self, condition: impl Fn(char) -> bool) {
        while self.next_if(&condition).is_some() {}
    }

    pub fn advance_past_next(&mut self, ch: char) {
        self.advance_while(|c| c != ch);
        if self.peek().is_some() {
            self.next();
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        let ch = *self.chars.get(self.cursor)?;
        Some(ch)
    }

    pub fn peek_next(&mut self) -> Option<char> {
        let ch = *self.chars.get(self.cursor + 1)?;
        Some(ch)
    }

    pub fn next_if(&mut self, condition: impl Fn(char) -> bool) -> Option<char> {
        let next = *self.chars.get(self.cursor)?;

        if !condition(next) {
            return None;
        }

        self.increment_cursor(next);

        Some(next)
    }

    pub fn next_map<T>(&mut self, map: impl Fn(char) -> Option<T>) -> Option<T> {
        let next = *self.chars.get(self.cursor)?;
        let result = map(next);
        if result.is_some() {
            self.increment_cursor(next);
        }
        result
    }

    pub fn next(&mut self) -> Option<char> {
        let next = *self.chars.get(self.cursor)?;

        self.increment_cursor(next);

        Some(next)
    }

    fn increment_cursor(&mut self, ch: char) {
        if ch == '\n' {
            self.line_position += 1;
            self.column_position = 1;
        } else {
            self.column_position += 1;
        }

        self.cursor += 1;
    }

    fn is_at_start_of_comment(&mut self) -> bool {
        self.peek() == Some('/') && self.peek_next() == Some('/')
    }

    fn is_next_char_whitespace(&mut self) -> bool {
        let Some(ch) = self.peek() else {
            return false;
        };
        ch.is_ascii_whitespace()
    }

    fn advance_to_next_line(&mut self) {
        self.advance_past_next('\n');
    }
}
