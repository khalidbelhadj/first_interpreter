use crate::token::Token;

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_numeric(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_numeric(c)
}

#[derive(Debug)]
pub struct Lexer {
    source: Box<[u8]>,
    pub tokens: Vec<Token>,
    current_idx: usize,
    current_line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.as_bytes().to_owned().into_boxed_slice(),
            tokens: vec![],
            current_idx: 0,
            current_line: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_idx >= self.source.len()
    }

    fn lex_token(&mut self) {
        let c = self.peek();

        match c {
            ' ' | '\r' | '\t' => {}
            '\n' => self.current_line += 1,
            '(' => self.tokens.push(Token::LeftParen),
            ')' => self.tokens.push(Token::RightParen),
            '{' => self.tokens.push(Token::LeftBrace),
            '}' => self.tokens.push(Token::RightBrace),
            ',' => self.tokens.push(Token::Comma),
            '.' => self.tokens.push(Token::Dot),
            '-' => self.tokens.push(Token::Minus),
            '+' => self.tokens.push(Token::Plus),
            '*' => self.tokens.push(Token::Star),
            ';' => self.tokens.push(Token::Semicolon),
            '/' => {
                self.current_idx += 1;
                match self.peek() {
                    '/' => {
                        self.current_idx += 1;
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.current_idx += 1;
                        }
                    }
                    _ => {
                        self.tokens.push(Token::Slash);
                    }
                }
            }
            '"' => {
                self.current_idx += 1;
                let token = self.lex_string();
                self.tokens.push(token);
                return;
            }
            '!' => {
                self.current_idx += 1;
                match self.peek() {
                    '=' => self.tokens.push(Token::BangEqual),
                    _ => self.tokens.push(Token::Bang),
                }
            }
            '=' => {
                self.current_idx += 1;
                match self.peek() {
                    '=' => self.tokens.push(Token::EqualEqual),
                    _ => self.tokens.push(Token::Equal),
                }
            }
            '>' => {
                self.current_idx += 1;
                match self.peek() {
                    '=' => self.tokens.push(Token::GreaterEqual),
                    _ => self.tokens.push(Token::Greater),
                }
            }
            '<' => {
                self.current_idx += 1;
                match self.peek() {
                    '=' => self.tokens.push(Token::LessEqual),
                    _ => self.tokens.push(Token::Less),
                }
            }
            _ => {
                if is_alpha(c) {
                    let mut token = self.lex_identifier();
                    assert!(matches!(token, Token::Identifier(_)));
                    token.try_convert_to_keyword();
                    self.tokens.push(token);
                    return;
                } else if is_numeric(c) {
                    let token = self.lex_number();
                    assert!(matches!(token, Token::Int(_) | Token::Float(_)));
                    self.tokens.push(token);
                    return;
                }
            }
        }
        self.current_idx += 1;
    }

    pub fn lex(&mut self) {
        while !self.is_at_end() {
            self.lex_token();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current_idx] as char;
    }

    fn lex_string(&mut self) -> Token {
        let mut string_literal = String::new();

        while self.peek() != '"' {
            if self.is_at_end() {
                eprintln!("Unterminated string literal on line {}", self.current_line);
                panic!();
            }

            string_literal.push(self.peek());

            if self.peek() == '\n' {
                eprintln!("String literal cannot contain newline");
            }

            self.current_idx += 1;
        }

        self.current_idx += 1;
        Token::String(string_literal)
    }

    fn lex_identifier(&mut self) -> Token {
        let mut identifier_literal = String::new();

        while is_alpha_numeric(self.peek()) {
            identifier_literal.push(self.peek());
            self.current_idx += 1;
        }

        Token::Identifier(identifier_literal)
    }

    fn lex_number(&mut self) -> Token {
        let mut number_literal = String::new();

        // First half of the float
        while is_numeric(self.peek()) {
            number_literal.push(self.peek());
            self.current_idx += 1;
        }

        // Decimal point means it's a float
        if self.peek() == '.' {
            number_literal.push(self.peek());
            self.current_idx += 1;

            // Second half of the float
            while is_numeric(self.peek()) {
                number_literal.push(self.peek());
                self.current_idx += 1;
            }

            return Token::Float(
                number_literal
                    .parse::<f64>()
                    .expect(format!("Could not parse number {number_literal}").as_str()),
            );
        }

        Token::Int(
            number_literal
                .parse::<i64>()
                .expect(format!("Could not parse number {number_literal}").as_str()),
        )
    }
}
