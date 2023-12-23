use crate::token::Token;
use crate::ast::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current_idx: usize,
    pub root: Program,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_idx: 0,
            root: Program { items: vec![] },
        }
    }

    fn advance(&mut self) {
        self.current_idx += 1;
    }

    fn retreat(&mut self) {
        self.current_idx -= 1;
    }

    fn peek_n(&self, n: usize) -> Token {
        if (self.current_idx + n) >= self.tokens.len() {
            return Token::EOF;
        }

        let token = self.tokens[self.current_idx + n].clone();
        token
    }

    fn peek(&self) -> Token {
        self.peek_n(0)
    }

    fn next_token(&mut self) -> Token {
        let token = self.peek();
        self.advance();
        token
    }

    pub fn parse(&mut self) {
        self.root = self.program();
    }

    fn program(&mut self) -> Program {
        let mut items = vec![];

        while self.current_idx < self.tokens.len() {
            items.push(self.item());
        }

        Program {
            items
        }
    }

    fn item(&mut self) -> Item {
        let token = self.peek();
        match token {
            | Token::Var
            | Token::Const
            | Token::Proc => Item::Declaration(self.declaration()),
            | Token::Return
            | Token::Identifier(_) => {
                let next_token = self.peek_n(1);
                match next_token {
                    Token::Equal => Item::Assignment(self.assignment()),
                    _ => Item::Expression(self.expression())
                }
            },
            _ => panic!("Expected item, got {:?}", token),
        }
    }

    fn declaration(&mut self) -> Declaration {
        let token = self.peek();
        match token {
            Token::Var => Declaration::Var(self.var()),
            Token::Const => Declaration::Const(self.cconst()),
            Token::Proc => Declaration::Proc(self.proc()),
            _ => panic!("Expected var or proc, got {:?}", token),
        }
    }

    fn var(&mut self) -> Var {
        let var = self.next_token();
        if !matches!(var, Token::Var) {
            panic!("Expected var, got {:?}", var);
        }

        let ident_token = self.next_token();
        let identifier: Option<String> = match ident_token {
            Token::Identifier(i) => Some(i),
            _ => panic!("Expected identifier, got {:?}", ident_token),
        };

        let equal = self.next_token();
        if !matches!(equal, Token::Equal) {
            panic!("Expected equal, got {:?}", equal);
        }

        Var {
            identifier: identifier.unwrap(),
            expr: self.expression(),
        }
    }

    fn cconst(&mut self) -> Const {
        let cconst = self.next_token();
        if !matches!(cconst, Token::Const) {
            panic!("Expected var, got {:?}", cconst);
        }

        let ident_token = self.next_token();
        let identifier: Option<String> = match ident_token {
            Token::Identifier(i) => Some(i),
            _ => panic!("Expected identifier, got {:?}", ident_token),
        };

        let equal = self.next_token();
        if !matches!(equal, Token::Equal) {
            panic!("Expected equal, got {:?}", equal);
        }

        Const {
            identifier: identifier.unwrap(),
            expr: self.expression(),
        }
    }

    fn proc(&mut self) -> Proc {
        let proc = self.next_token();
        if !matches!(proc, Token::Proc) {
            panic!("Expected proc, got {:?}", proc);
        }

        let ident_token = self.next_token();
        let identifier: Option<String> = match ident_token {
            Token::Identifier(i) => Some(i),
            _ => panic!("Expected identifier, got {:?}", ident_token),
        };

        let args = self.proc_args();

        Proc {
            identifier: identifier.unwrap(),
            proc_args: args,
            block: self.block(),
        }
    }

    fn proc_args(&mut self) -> ProcArgs {
        let left_paren = self.next_token();
        if !matches!(left_paren, Token::LeftParen) {
            panic!("Expected '(', got {:?}", left_paren);
        }

        let mut args: ProcArgs = vec![];

        loop {
            let token = self.next_token();
            match token.clone() {
                Token::Identifier(name) => args.push(name),
                Token::RightParen => break,
                _ => panic!("Expected identifier, got {:?}", token),
            }

            let next_token = self.next_token();
            match next_token {
                Token::Comma => {}
                Token::RightParen => break,
                _ => panic!("Expected ',' or ')', got {:?}", next_token),
            }
        }

        args
    }

    fn assignment(&mut self) -> Assignment {
        let ident_token = self.next_token();
        let identifier: Option<String> = match ident_token {
            Token::Identifier(i) => Some(i),
            _ => panic!("Expected identifier, got {:?}", ident_token),
        };

        let equal = self.next_token();
        if !matches!(equal, Token::Equal) {
            panic!("Expected equal, got {:?}", equal);
        }

        Assignment {
            identifier: identifier.unwrap(),
            expr: self.expression(),
        }
    }


    fn block(&mut self) -> Block {
        let left_brace = self.next_token();
        if !matches!(left_brace, Token::LeftBrace) {
            panic!("Expected left brace, got {:?}", left_brace);
        }

        let mut items = vec![];
        while self.current_idx < self.tokens.len() {
            let token = self.peek();
            match token {
                Token::RightBrace => break,
                _ => {
                    items.push(self.item());
                }
            }
        }
        // Get rid of the remaining right brace
        self.advance();

        Block {
            items
        }
    }

    fn expression(&mut self) -> Expression {
        let left = self.unary();

        let token = self.peek();
        if !matches!(token, Token::Plus | Token::Minus | Token::Star | Token::Slash) {
            return Expression::Unary(left);
        }

        self.retreat();
        return Expression::Binary(self.binary());
    }

    fn binary(&mut self) -> Binary {
        let left = self.unary();
        let mut binary: Option<Binary> = None;

        loop {
            let token = self.next_token();
            let operator = match token {
                Token::Plus => BinaryOperator::Plus,
                Token::Minus => BinaryOperator::Minus,
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                _ => {
                    self.retreat();
                    break
                },
            };

            let new_left = left.clone();
            match binary {
                Some(_) => {
                    // TODO implement me
                },
                None => {
                    binary = Some(Binary {
                        left: new_left,
                        operator,
                        right: Box::new(self.expression())
                    })
                }
            }
        }

        binary.expect("idk what to tell you bruh")
    }

    fn unary(&mut self) -> Unary {
        let token = self.next_token();

        match token {
            Token::Bang => Unary::UnaryOperation {
                operator: UnaryOperator::Negate,
                unary: Box::new(self.unary()),
            },
            _ => {
                self.retreat();
                Unary::Call(self.call())
            },
        }
    }

    fn call(&mut self) -> Call {
        let token = self.next_token();
        match token {
            | Token::True
            | Token::False
            | Token::Null
            | Token::Int(_)
            | Token::Float(_)
            | Token::String(_)
            | Token::Return
            | Token::LeftParen => {
                self.retreat();
                Call::Primary(self.primary())
            }
            Token::Identifier(name) => {
                let next_token = self.peek();
                let identifier = name;
                match next_token {
                    Token::LeftParen => {
                        Call::CallLiteral {
                            identifier,
                            call_args: self.call_args()
                        }
                    }
                    _ => Call::Primary(Primary::Identifier(identifier))
                }
            }
            _ => panic!("Expected expression, got {:?}", token),
        }
    }

    fn primary(&mut self) -> Primary {
        let value = self.next_token();
        match value {
            Token::True => Primary::True,
            Token::False => Primary::False,
            Token::Null => Primary::Null,
            Token::Int(value) => Primary::Int(value),
            Token::Float(value) => Primary::Float(value),
            Token::String(value) => Primary::String(value),
            Token::Identifier(name) => Primary::Identifier(name),
            Token::Return => {
                let expression = self.expression();
                Primary::Return(Box::new(expression))
            },
            Token::LeftParen => {
                let expression = self.expression();

                let right_paren = self.next_token();
                if !matches!(right_paren, Token::RightParen) {
                    panic!("Expected left brace, got {:?}", right_paren);
                }

                Primary::Expression(Box::new(expression))
            }
            _ => panic!("Expected primary, got {:?}", value),
        }
    }

    fn call_args(&mut self) -> CallArgs {
        let left_paren = self.next_token();
        if !matches!(left_paren, Token::LeftParen) {
            panic!("Expected '(', got {:?}", left_paren);
        }

        let mut args: CallArgs = vec![];

        loop {
            let token = self.peek();
            match token.clone() {
                Token::RightParen => {
                    self.next_token();
                    break;
                }
                _ => {
                    args.push(self.expression());
                }
            }

            let next_token = self.next_token();
            match next_token {
                Token::Comma => {}
                Token::RightParen => break,
                _ => panic!("Expected ',' or ')', got {:?}", next_token),
            }
        }

        args
    }
}
