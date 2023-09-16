use crate::opcodes::Opcode;

#[derive(Debug)]
pub struct Stack {
    pub values: Vec<Vec<String>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    pub fn update(&mut self, opcode: Opcode) {
        if opcode.pops == 1 && opcode.pushes == 1 {
            let popped = self.peek().unwrap().clone();
            let result = format!("{}: {}", opcode.name, popped);
            self.pop_and_push(result);
            return;
        }
        if opcode.pops > 0 {
            self.pop(opcode.pops);
        }
        if opcode.pops == 0 && opcode.pushes == 0 {
            self.dup_last();
        }
    }

    pub fn dup_last(&mut self) {
        let last_values = self.values.last().unwrap().clone();
        self.values.push(last_values);
    }

    pub fn push(&mut self, value: String) {
        if self.values.len() == 0 {
            self.values.push(vec![value]);
        } else {
            let mut last_values = self.values.last().unwrap().clone();
            println!("last_values: {:?}", last_values);
            last_values.push(value);
            self.values.push(last_values);
        }
    }

    pub fn push_takes(&mut self, takes: i32) {
        if takes > 0 {
            let mut takes_vec = Vec::new();
            for i in 0..takes {
                takes_vec.push(format!("a{}", i));
            }
            self.values.push(takes_vec);
        }
    }

    pub fn pop(&mut self, pops: usize) {
        if self.values.len() == 0 {
            return;
        }

        let mut last_values = self.values.last().unwrap().clone();
        for _ in 0..pops {
            last_values.pop();
        }
        self.values.push(last_values);
    }

    pub fn pop_and_push(&mut self, value: String) {
        if self.values.len() == 0 {
            return;
        }
        let mut last_values = self.values.last().unwrap().clone();
        last_values.pop();
        last_values.push(value);
        self.values.push(last_values);
    }

    pub fn peek(&self) -> Option<&String> {
        self.values[self.values.len() - 1].last()
    }
}
