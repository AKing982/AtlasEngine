use std::collections::HashMap;

use crate::token_type::TokenType;
use crate::token::Token;
use crate::literal::Literal;

pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: Self::initialize_keywords(),
        }
    }

    pub(crate) fn initialize_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("fn".to_string(), TokenType::Fn);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("sin".to_string(), TokenType::Sin);
        keywords.insert("cos".to_string(), TokenType::Cos);
        keywords.insert("tan".to_string(), TokenType::Tan);
        keywords.insert("exp".to_string(), TokenType::Exp);
        keywords.insert("log".to_string(), TokenType::Log);
        keywords.insert("sqrt".to_string(), TokenType::Sqrt);
        keywords.insert("complex".to_string(), TokenType::Complex);
        keywords.insert("conjugate".to_string(), TokenType::Conjugate);
        keywords.insert("limit".to_string(), TokenType::Limit);
        keywords.insert("i".to_string(), TokenType::I);
        keywords.insert("pi".to_string(), TokenType::Pi);
        keywords.insert("e".to_string(), TokenType::E);
        keywords
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end(){
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
        });
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let token_type = self.keywords
            .get(&text)
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    fn is_alpha(character: char) -> bool {
        character.is_ascii_alphabetic() || character == '_'
    }

    fn is_alpha_numeric(character: char) -> bool {
        Self::is_alpha(character) || Self::is_digit(character)
    }

    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end(){
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn scan_token(&mut self) {
        let char = self.advance();
        match char {
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '^' => self.add_token(TokenType::Caret),
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                }else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('='){
                    TokenType::GreaterEqual
                }else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                }else{
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            ',' => self.add_token(TokenType::Comma),
            '.' => {
                if Self::is_digit(self.peek()){
                    self.number()
                }else {
                    self.add_token(TokenType::Dot);
                }
            }
            '/' => {
                if self.match_char('/'){
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }else{
                    self.add_token(TokenType::Slash);
                }
            }
            ';' => {
                self.add_token(TokenType::SemiColon)
            }
            ' ' | '\r' | '\t' => {
                // Ignore Whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            c if Self::is_digit(c) => self.number(),
            c if Self::is_alpha(c) => self.identifier(),
            _ => {

            }
        }
    }

    fn is_digit(character: char) -> bool {
        character.is_ascii_digit()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            }
            if self.is_at_end(){
                panic!("Undetermined String");
            }
            self.advance();
            let value = self.source[self.start + 1..self.current - 1].to_string();
            self.add_string_token(TokenType::String, value);
        }
    }

    fn number(&mut self) {
        let starts_with_dot = self.source[self.start..].starts_with(".");
        if !starts_with_dot {
            while Self::is_digit(self.peek()){
                self.advance();
            }
            if self.peek() == '.' && Self::is_digit(self.peek_next()){
                self.advance();
            }
        }else {
            self.advance();
        }
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        let text = if starts_with_dot {
            format!("0{}", &self.source[self.start..self.current])
        }else{
            self.source[self.start..self.current].to_string()
        };
        let value: f64 = text.parse().unwrap();
        self.add_number_token(TokenType::Number, value);
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len(){
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn add_number_token(&mut self, token_type: TokenType, value: f64) {
        let lexeme = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            token_type,
            lexeme,
            literal: Some(Literal::Number(value)),
        });
    }

    fn add_string_token(&mut self, token_type: TokenType, value: String){
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token{
            token_type,
            lexeme,
            literal: Some(Literal::String(value))
        });
    }

    fn peek(&mut self) -> char {
        if self.is_at_end(){
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal:None,
        });
    }

}