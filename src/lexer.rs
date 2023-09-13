use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::printer::Printer;
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
        stack.pop_and_push("calldata".to_string());
    }

    if l.starts_with("[") {
        stack.push("ptr".to_string());
    }

    if l == "sstore" {
        stack.pop2();
    }
    if l == "mstore" {
        stack.pop2();
    }
    if l == "mload" {
        let popped = stack.peek().unwrap().clone();
        let result = format!("mload: {}", popped);
        stack.pop_and_push(result.to_string());
    }
}

fn parse_macro(contents: String) -> (String, usize) {
    let mut macro_contents = String::new();

    let mut start = 0;
    let mut in_macro = false;
    for (i, l) in contents.lines().enumerate() {
        // in macro
        if in_macro && !l.starts_with("}") {
            // add line to macro_contents with a new line
            macro_contents.push_str(l);
            macro_contents.push_str("\n");
        }

        // start of macro
        if !in_macro && l.starts_with("#define macro") {
            start = i;
            in_macro = true;
        }

        // end of macro
        if in_macro && l.starts_with("}") {
            in_macro = false;
        }
    }

    return (macro_contents, start)
}

fn replace_macro(start: usize, comments: String, contents: String) -> String {
    // put lines into a vector
    let mut content_lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();


    let mut ii = 0;
    //replace
    for index in start+1..=comment_lines.len()+1 {
        content_lines[index] = comment_lines[ii].clone();
        ii += 1;
    }

    // content lines to string with new line
    let mut final_text = String::new();
    for line in content_lines {
        final_text.push_str(line.as_str());
        final_text.push_str("\n");
    }

    // println!("{}", final_text);

    return final_text;

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
        let (macro_contents, start) = parse_macro(contents.clone());

        let mut stack = Stack::new();

        let mut longest_line = 0;

        for l in macro_contents.lines() {
            if l.len() > longest_line {
                longest_line = l.len();
            }
            parse_opcode(&mut stack, l.to_string());
        }

        let printer = Printer::new(macro_contents, stack, longest_line);
        let comments = printer.print();

        let with_comments = replace_macro(start, comments, contents);

        // write comments to a file
        let mut file = File::create("output.huff").expect("Unable to create file");
        file.write_all(with_comments.as_bytes())
            .expect("Unable to write data");
    }
}
