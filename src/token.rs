#[derive(Debug, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Const,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    String(String),
    Int(i64),
    Float(f64),
    And,
    Else,
    False,
    For,
    If,
    Null,
    Or,
    Proc,
    Return,
    Struct,
    True,
    Var,
    While,
    EOF,
}

impl Token {
    pub fn try_convert_to_keyword(&mut self) {
        if let Token::Identifier(literal) = self {
            *self = match literal.as_str() {
                "and"    => Token::And,
                "const"  => Token::Const,
                "else"   => Token::Else,
                "false"  => Token::False,
                "for"    => Token::For,
                "if"     => Token::If,
                "null"   => Token::Null,
                "or"     => Token::Or,
                "proc"   => Token::Proc,
                "return" => Token::Return,
                "struct" => Token::Struct,
                "true"   => Token::True,
                "var"    => Token::Var,
                "while"  => Token::While,
                _        => Token::Identifier(literal.to_string())
            }
        }
    }

}
