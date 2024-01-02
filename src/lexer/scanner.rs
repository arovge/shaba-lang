use super::token::SourceLocation;

pub struct Scanner {
    chars: Vec<char>,
    cursor: usize,
    line_position: usize,
    column_position: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Self {
            chars,
            cursor: 0,
            line_position: 1,
            column_position: 1,
        }
    }

    pub fn location(&self) -> SourceLocation {
        SourceLocation::new(self.line_position, self.column_position)
    }

    pub fn is_eof(&self) -> bool {
        self.peek().is_none()
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
        loop {
            if self.next_if(&condition).is_none() {
                break;
            }
        }
    }

    pub fn advance_past_next(&mut self, ch: char) {
        self.advance_while(|c| c != ch);
        if self.peek().is_some() {
            self.next();
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.cursor).map(|c| c.to_owned())
    }

    pub fn peek_next(&self) -> Option<char> {
        self.chars.get(self.cursor + 1).map(|c| c.to_owned())
    }

    pub fn next_if(&mut self, condition: impl Fn(char) -> bool) -> Option<char> {
        self.next_if_map(|x| if condition(x.clone()) { Some(x) } else { None })
    }

    pub fn next_if_map<T>(&mut self, map: impl Fn(char) -> Option<T>) -> Option<T> {
        let next = self.peek()?;
        let result = map(next);
        if result.is_some() {
            self.increment_cursor(next);
        }
        result
    }

    pub fn next(&mut self) -> Option<char> {
        let next = self.peek()?;

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

    fn is_at_start_of_comment(&self) -> bool {
        let cond = |c: char| c == '/';
        self.peek()
            .map(cond)
            .and_then(|_| self.peek_next().map(cond))
            .unwrap_or(false)
    }

    fn is_next_char_whitespace(&self) -> bool {
        self.peek()
            .map(|c| c.is_ascii_whitespace())
            .unwrap_or(false)
    }

    fn advance_to_next_line(&mut self) {
        self.advance_past_next('\n');
    }
}
