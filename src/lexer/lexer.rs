use super::{
    token::{Keyword, Literal},
};
use crate::lexer::token::Token;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        while self.position < self.source.len() {
            let token = self.next_token();
            tokens.push(token);
        }

        return tokens;
    }

    fn next_token(&mut self) -> Token {
        // TODO: Return whitespace tokens?
        self.eat_if_whitespace();

        if let Some(token) = self.read_single_char_token() {
            return token;
        }

        if let Some(literal) = self.read_literal() {
            return literal.into();
        }

        if let Some(id) = self.read_identifier() {
            return match id.as_str() {
                "let" => Token::Keyword(Keyword::Let),
                "var" => Token::Keyword(Keyword::Var),
                _ => Token::Identifier(id.to_string()),
            };
        }
        let ch = self.char();
        self.eat_char();
        if let Some(ch) = ch {
            return Token::Unknown(ch.to_string());
        } else {
            return Token::EOF;
        }
    }

    fn read_identifier(&mut self) -> Option<String> {
        if !is_start_of_identifier(self.char()?) {
            return None;
        }

        let start_position = self.position;
        while let Some(ch) = self.char() {
            if !is_identifier(ch) {
                break;
            }
            self.eat_char();
        }
        let chars = self.source[start_position..self.position].to_vec();
        return Some(String::from_iter(chars));
    }

    fn read_literal(&mut self) -> Option<Literal> {
        if let Some(str) = self.read_str() {
            return Some(Literal::String(str));
        }
        if let Some(bool) = self.read_bool() {
            return Some(Literal::Bool(bool));
        }

        // TODO: number parsing
        return None;
    }

    fn read_single_char_token(&mut self) -> Option<Token> {
        let token = match self.char()? {
            '=' => Some(Token::Equals),
            '{' => Some(Token::LeftCurlyBrace),
            '}' => Some(Token::RightCurlyBrace),
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            ',' => Some(Token::Comma),
            ';' => Some(Token::Semicolon),
            ':' => Some(Token::Colon),
            ' ' => Some(Token::Space),
            _ => None,
        }?;
        self.eat_char();
        return Some(token);
    }

    fn read_str(&mut self) -> Option<String> {
        if self.char()? != '"' {
            return None;
        }
        let start_position = self.position;
        self.position += 1;
        self.eat_until(|c| c == '"');

        let chars = self.source[start_position..self.position].to_vec();
        let str = String::from_iter(chars);
        return Some(str);
    }

    fn read_bool(&mut self) -> Option<bool> {
        let true_len = "true".len();
        let false_len = "false".len();

        // TODO: This doesn't check to see if the character after "true" or "false" is whitespace (or semicolon)
        if self.position + false_len < self.source[self.position..].len() {
            let chars = &self.source[self.position..(self.position + false_len)];
            let str = String::from_iter(chars);
            if str.eq("false") {
                self.position += false_len;
                return Some(false);
            }
        }

        if self.position + true_len < self.source[self.position..].len() {
            let chars = &self.source[self.position..(self.position + true_len)];
            let str = String::from_iter(chars);
            if str.eq("true") {
                self.position += true_len;
                return Some(true);
            }
        }

        return None;
    }

    fn eat_if_whitespace(&mut self) {
        while let Some(ch) = self.char() {
            if ch.is_ascii_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn eat_char(&mut self) {
        if self.position >= self.source.len() {
            return;
        }
        self.position += 1;
    }

    fn eat_until(&mut self, condition: fn(char) -> bool) {
        while let Some(ch) = self.char() {
            self.position += 1;
            if condition(ch) {
                break;
            }
        }
    }

    fn char(&self) -> Option<char> {
        self.source.get(self.position).map(|ch| ch.clone())
    }
}

fn is_start_of_identifier(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier(c: char) -> bool {
    is_start_of_identifier(c) || c.is_ascii_digit()
}
