use crate::opcodes::Opcode;
use crate::function::Function;
use crate::parser::{parse_line};
use crate::token::{Token, TokenType};

const COMMENT_START: &str = "//";
const CONSTANT_START: &str = "0x";
const REFERENCE_START: &str = "[";
const VARIABLE_START: &str = "<";

// We insert this placeholder into a function if it takes more than 0 arguments.
// The stack usese this placeholder to determine where to insert the arguments.
pub const TAKES_PLACEHOLDER: &str = "$takes$";

#[derive(Debug)]
pub struct StackHistory {
    pub stacks: Vec<Stack>,
}

impl StackHistory {
    pub fn new() -> StackHistory {
        StackHistory { stacks: Vec::new() }
    }

    pub fn push(&mut self, stack: Stack) {
        self.stacks.push(stack);
    }

    pub fn peek(&self) -> Option<&Stack> {
        self.stacks.last()
    }
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub values: Vec<Token>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    pub fn update(&mut self, token: Token) {
        let opcode = token.opcode.as_ref().unwrap();
        for _ in 0..opcode.pops {
            self.pop();
        }
        if opcode.pushes == 1 {
            self.push(token);
        }
    }

    pub fn push(&mut self, value: Token) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<Token> {
        self.values.pop()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.values.last()
    }

    pub fn push_takes(&mut self, takes: i32) {
        if takes > 0 {
            for i in 0..takes {
                let mut token = Token::new();
                token.token_type = TokenType::Constant;
                self.values.push(token);
            }
        }
    }
}
