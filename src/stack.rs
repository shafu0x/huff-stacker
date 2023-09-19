use crate::function::Function;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub values: Vec<Token>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    pub fn push_takes(&mut self, takes: i32) {
        if takes > 0 {
            for i in 0..takes {
                let mut token = Token::new();
                token.value = format!("${}", i);
                token.token_type = TokenType::Constant;
                self.push(token);
            }
        }
    }

    pub fn execute_opcode(&mut self, token: Token) {
        let opcode = token.opcode.as_ref().unwrap();

        for _ in 0..opcode.pops {
            self.pop().unwrap();
        }

        if opcode.pushes == 1 {
            self.push(token);
        }
    }

    pub fn execute_function(&mut self, function: &Function) {
        for _ in 0..function.takes {
            self.pop().unwrap();
        }

        for i in 0..function.returns {
            let mut token = Token::new();
            token.value = format!("{}: %{}", function.name, i);
            token.token_type = TokenType::Return;
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
}
