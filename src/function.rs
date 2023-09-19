use crate::parser::parse_line;
use crate::stack::{Stack, StackHistory};
use crate::token::TokenType;

const COMMENT_START: &str = "//";

pub struct Function {
    pub start: usize, // The line number where the function starts
    pub end: usize,   // The line number where the function ends
    pub takes: i32,   // The number of arguments the function takes
    pub body: String, // The body of the function
    pub stack_history: StackHistory,
}

impl Function {
    pub fn new() -> Function {
        Function {
            start: 0,
            end: 0,
            takes: 0,
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
