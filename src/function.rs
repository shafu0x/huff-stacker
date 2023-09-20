use crate::jump::JumpLabelsMap;
use crate::parser::parse_line;
use crate::stack::{Stack, StackHistory};
use crate::token::TokenType;
use std::collections::HashMap;

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
    pub jump_labels_map: JumpLabelsMap,
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
            jump_labels_map: JumpLabelsMap::new(),
        }
    }

    // create a function that is built into huff
    pub fn builtin(name: &str) -> Function {
        let mut function = Function::new();
        function.name = name.to_string();
        function.takes = 1;
        function.returns = 1;
        function
    }

    pub fn longest_line(&self) -> usize {
        self.body.lines().map(|line| line.len()).max().unwrap_or(0)
    }

    pub fn gen_jump_labels(&mut self) {
        let mut jump_labels_map = JumpLabelsMap::new();
        for line in self.body.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.ends_with(":") {
                jump_labels_map.add(trimmed_line.to_string());
            }
        }
        self.jump_labels_map = jump_labels_map;
    }

    pub fn gen_stack_history(&mut self, functions_map: &FunctionsMap) {
        let mut stack_history = StackHistory::new();
        let mut stack = Stack::new();
        stack.push_takes(self.takes);
        for line in self.body.lines() {
            if !line.trim().starts_with(COMMENT_START) {
                let tokens = parse_line(line);
                for mut token in tokens {
                    match token.token_type {
                        TokenType::Constant | TokenType::Reference | TokenType::Variable => {
                            stack.push(token);
                        }
                        TokenType::Function => {
                            stack.execute_function(functions_map.get(&token.value));
                        }
                        TokenType::Opcode => {
                            // IMPORTANT: We need to set the operands before executing the opcode
                            token.set_operands(&stack);
                            stack.execute_opcode(token);
                        }
                        _ => {}
                    }
                }
            }
            stack_history.push(stack.clone());
        }
        self.stack_history = stack_history
    }
}

#[derive(Debug)]
pub struct FunctionsMap {
    pub map: HashMap<String, Function>,
}

impl FunctionsMap {
    pub fn new() -> Self {
        FunctionsMap { map: Self::init() }
    }

    // add all the built-in huff functions to the map
    fn init() -> HashMap<String, Function> {
        let mut map = HashMap::new();

        let func_sig = Function::builtin("__FUNC_SIG");
        let event_hash = Function::builtin("__EVENT_HASH");
        let error = Function::builtin("__ERROR");
        let right_pad = Function::builtin("__RIGHT_PAD");
        let codesize = Function::builtin("__CODESIZE");
        let tablestart = Function::builtin("__TABLESTART");
        let tablesize = Function::builtin("__TABLESIZE");

        map.insert(func_sig.name.clone(), func_sig);
        map.insert(event_hash.name.clone(), event_hash);
        map.insert(error.name.clone(), error);
        map.insert(right_pad.name.clone(), right_pad);
        map.insert(codesize.name.clone(), codesize);
        map.insert(tablestart.name.clone(), tablestart);
        map.insert(tablesize.name.clone(), tablesize);

        map
    }

    pub fn add(&mut self, function: Function) {
        if self.map.contains_key(&function.name) {
            panic!("Function {} already exists", function.name);
        }
        self.map.insert(function.name.clone(), function);
    }

    pub fn get(&self, name: &str) -> &Function {
        self.map
            .get(name)
            .expect(&format!("ERROR: Function {} not found", name))
    }
}
