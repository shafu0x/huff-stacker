use std::collections::HashMap;
use crate::parser::parse_line;
use crate::stack::{Stack, StackHistory};
use crate::token::TokenType;

const COMMENT_START: &str = "//";

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub start: usize, // line number where the function starts
    pub end: usize,   // line number where the function ends
    pub takes: i32,   // number of arguments the function takes
    pub returns: i32, // number of arguments the function returns
    pub body: String, // body of the function
    pub stack_history: StackHistory,
}

impl Function {
    pub fn new() -> Function {
        Function {
            name: String::new(),
            start: 0,
            end: 0,
            takes: 0,
            returns: 0,
            body: String::new(),
            stack_history: StackHistory::new(),
        }
    }

    pub fn longest_line(&self) -> usize {
        self.body.lines().map(|line| line.len()).max().unwrap_or(0)
    }

    pub fn gen_stack_history(&mut self) {
        let mut stack_history = StackHistory::new();
        let mut stack = Stack::new();
        stack.push_takes(self.takes);
        for line in self.body.lines() {
            if !line.trim().starts_with(COMMENT_START) {
                let tokens = parse_line(line);
                for mut token in tokens {
                    if token.token_type == TokenType::Constant {
                        stack.push(token);
                    } else if token.token_type == TokenType::Reference {
                        stack.push(token);
                    } else if token.token_type == TokenType::Variable {
                        stack.push(token);
                    } else if token.token_type == TokenType::Opcode {
                        // IMPORTANT: We need to set the operands before executing the opcode
                        token.set_operands(&stack);
                        stack.execute(token);
                    }
                }
            }
            stack_history.push(stack.clone());
        }
        self.stack_history = stack_history
    }
}

#[derive(Debug)]
pub struct FunctionTable {
    pub table: HashMap<String, Function>,
}

impl FunctionTable {
    pub fn new() -> Self {
        FunctionTable {
            table: HashMap::new(),
        }
    }

    pub fn add(&mut self, function: Function) {
        if self.table.contains_key(&function.name) {
            panic!("Function {} already exists", function.name);
        }
        self.table.insert(function.name.clone(), function);
    }
}
