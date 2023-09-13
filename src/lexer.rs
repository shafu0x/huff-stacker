use std::fs::File;
use std::io::Read;

use crate::stack::Stack;

pub struct Lexer {
    path: String,
}

fn parse_opcode(stack: &mut Stack, line: String) {
    let l = line.trim();

    if l.starts_with("0x") {
        stack.push(l.to_string());
    }

    if l == "calldataload" {
        stack.pop();
        stack.push("calldata".to_string());
    }

    if l.starts_with("[") {
        stack.push("[VALUE_LOCATION]".to_string());
    }

    if l == "sstore" {
        stack.pop();
        stack.pop();
    }
}

fn parse_macro(contents: String) -> String {
    let mut macro_contents = String::new();

    let mut in_macro = false;
    for l in contents.lines() {
        // in macro
        if in_macro && !l.starts_with("}") {
            // add line to macro_contents with a new line
            macro_contents.push_str(l);
            macro_contents.push_str("\n");
        }

        // start of macro
        if !in_macro && l.starts_with("#define macro") {
            in_macro = true;
        }

        // end of macro
        if in_macro && l.starts_with("}") {
            in_macro = false;
        }
    }

    macro_contents
}

impl Lexer {
    pub fn new(path: String) -> Lexer {
        Lexer { path: path }
    }

    pub fn read_file(&self) {
        let mut file = File::open(self.path.as_str()).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error reading file");

        // TODO: refactor the clone
        let macro_contents = parse_macro(contents.clone());
        // println!("{}", macro_contents);

        let mut stack = Stack::new();

        for l in macro_contents.lines() {
            parse_opcode(&mut stack, l.to_string());
            println!("{}", stack);
        }
    }
}
