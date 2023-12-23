pub type Identifier = String;

#[derive(Debug, Clone)]
pub enum Primary {
    True,
    False,
    Null,
    Float(f64),
    Int(i64),
    String(String),
    Identifier(Identifier),
    Return(Box<Expression>),
    Expression(Box<Expression>),
}

pub type CallArgs = Vec<Expression>;

#[derive(Debug, Clone)]
pub enum Call {
    Primary(Primary),
    CallLiteral {
        identifier: Identifier,
        call_args: CallArgs,
    },
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
}

#[derive(Debug, Clone)]
pub enum Unary {
    Call(Call),
    UnaryOperation {
        operator: UnaryOperator,
        unary: Box<Unary>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Unary,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Unary(Unary),
    Binary(Binary),
}

pub type Block = Program;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub identifier: Identifier,
    pub expr: Expression,
}

pub type ProcArgs = Vec<Identifier>;

#[derive(Debug, Clone)]
pub struct Proc {
    pub identifier: Identifier,
    pub proc_args: ProcArgs,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct Const {
    pub identifier: Identifier,
    pub expr: Expression,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub identifier: Identifier,
    pub expr: Expression,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Var(Var),
    Const(Const),
    Proc(Proc),
}

#[derive(Debug, Clone)]
pub enum Item {
    Assignment(Assignment),
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

