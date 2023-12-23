use std::collections::HashMap;
use crate::ast::*;

type Address = u32;

const STDIN: u32 = 0;
const STDOUT: u32 = 1;

#[derive(Copy, Clone)]
pub enum SysCall {
    Read {
        file_descriptor: u32,
        buf_pointer: u32,
        count: u32
    },
    Write {
        file_descriptor: u32,
        buf_pointer: u32,
        count: u32
    },
}

#[derive(Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D
}

#[derive(Clone)]
pub enum Instruction {
    Noop,
    Ret,
    Move(Register, u32), // Move(to, value)

    // Jump instructions
    Jump(Address),
    Call(String),
    JEQ(u32, u32, Address), // If a == b, jump to address
    JGT(u32, u32, Address), // If a > b, jump to address
    JGE(u32, u32, Address), // If a >= b, jump to address

    // Memory instructions
    LA(Register, String), // register = &string
    LW(Register, Address), // register = *address
    SW(Register, Address), // *address = register
    Push(u32), // stack.push(value)
    Pop(Register), // register = stack.pop()

    // SysCall(SysCall)
}

pub struct VM {
    instructions: Vec<Instruction>,
    labels: HashMap<String, Address>,
    memory: Vec<Address>,
    ret: Vec<u32>,
    ip: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl VM {
    pub fn new() -> Self {
        VM {
            instructions: vec![],
            labels: HashMap::new(),
            memory: vec![],
            ret: vec![],
            ip: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }

    pub fn from_ast(root: Program) -> Self {

        let mut vm = VM::new();

        for item in root.items {
            match item {
                Item::Assignment(ass) => {
                    let Assignment { identifier, expr } = ass;
                    todo!()
                }
                Item::Declaration(decl) => {
                    match decl {
                        Declaration::Var(var) => {
                            let Var {identifier, expr} = var;
                            todo!()
                        }
                        Declaration::Const(const_) => {
                            let Const {identifier, expr} = const_;
                            todo!()
                        }
                        Declaration::Proc(proc) => {
                            let Proc {identifier, proc_args, block} = proc;
                            todo!()
                        }
                    }
                }
                Item::Expression(expr) => {
                    match expr {
                        Expression::Unary(unary) => {
                            match unary {
                                Unary::Call(call) => {
                                    match call {
                                        Call::Primary(primary) => {
                                            match primary {
                                                Primary::True => {
                                                    todo!()
                                                }
                                                Primary::False => {
                                                    todo!()
                                                }
                                                Primary::Null => {
                                                    todo!()
                                                }
                                                Primary::Float(float) => {
                                                    todo!()
                                                }
                                                Primary::Int(int) => {
                                                    todo!()
                                                }
                                                Primary::String(string) => {
                                                    todo!()
                                                }
                                                Primary::Identifier(ident) => {
                                                    todo!()
                                                }
                                                Primary::Return(expr) => {
                                                    todo!()
                                                }
                                                Primary::Expression(expr) => {
                                                    todo!()
                                                }
                                            }
                                        }
                                        Call::CallLiteral {identifier, call_args } => {
                                            todo!()
                                        }
                                    }
                                }
                                Unary::UnaryOperation { unary, operator} => {
                                    todo!()
                                }
                            }
                        }
                        Expression::Binary(binary) => {
                            let Binary { operator, left, right} = binary;
                            todo!()
                        }
                    }
                }
            }
        }
        todo!()
    }

    pub fn next_instruction(&mut self) -> Option<Instruction> {
        if self.ip >= self.instructions.len() as u32 {
            return None;
        }

        let instruction = self.instructions[self.ip as usize].clone();
        self.ip += 1;
        return Some(instruction);
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.next_instruction();

            if instruction.is_none() {
                break;
            }

            match instruction.unwrap() {
                Instruction::Noop => {},
                Instruction::Ret => {
                    let return_value = self.ret.pop();

                    match return_value {
                        Some(return_value) => self.ip = return_value,
                        None => break
                    }
                },
                // _ => todo!()
                Instruction::Move(to, value) => {
                    match to {
                        Register::A => self.a = value,
                        Register::B => self.b = value,
                        Register::C => self.c = value,
                        Register::D => self.d = value,
                    }
                }
                Instruction::Jump(address) => {
                    self.ip = address;
                }
                Instruction::Call(call_addr) => {
                    self.ret.push(self.ip);
                    self.ip = self.labels[&call_addr];
                }
                Instruction::JEQ(a, b, address) => {
                    if a == b {
                        self.ip = address;
                    }
                }
                Instruction::JGT(a, b, address) => {
                    if a > b {
                        self.ip = address;
                    }
                }
                Instruction::JGE(a, b, address) => {
                    if a >= b {
                        self.ip = address;
                    }
                }
                Instruction::LA(register, label) => {
                    match register {
                        Register::A => self.a = self.labels[&label],
                        Register::B => self.b = self.labels[&label],
                        Register::C => self.c = self.labels[&label],
                        Register::D => self.d = self.labels[&label],
                    }
                }
                Instruction::LW(register, address) => {
                    match register {
                        Register::A => self.a = self.memory[address as usize],
                        Register::B => self.b = self.memory[address as usize],
                        Register::C => self.c = self.memory[address as usize],
                        Register::D => self.d = self.memory[address as usize]
                    }
                }
                Instruction::SW(register, address) => {
                    match register {
                        Register::A => self.memory[address as usize] = self.a,
                        Register::B => self.memory[address as usize] = self.b,
                        Register::C => self.memory[address as usize] = self.c,
                        Register::D => self.memory[address as usize] = self.d
                    }
                }
                Instruction::Push(value) => self.memory.push(value),
                Instruction::Pop(register) => {
                    match register {
                        Register::A => self.a = self.memory.pop().unwrap(),
                        Register::B => self.b = self.memory.pop().unwrap(),
                        Register::C => self.c = self.memory.pop().unwrap(),
                        Register::D => self.d = self.memory.pop().unwrap()
                    }
                }
            }
        }
    }
}
