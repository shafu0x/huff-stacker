use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::printer::Printer;
use crate::stack::Stack;

pub struct Lexer {
    path: String,
}

fn parse_opcode(stack: &mut Stack, line: &str) {
    match line {
        "calldataload" => stack.pop_and_push("calldata".to_string()),
        "sstore" => stack.pop2(),
        "mstore" => stack.pop2(),
        "mload" => {
            let popped = stack.peek().unwrap().clone();
            let result = format!("mload: {}", popped);
            stack.pop_and_push(result.to_string());
        }
        &_ => todo!(),
    };
}

fn parse_line(stack: &mut Stack, line: String) {
    let trimmed_line = line.trim();

    match trimmed_line {
        line if line.starts_with("0x") => stack.push(line.to_string()),
        line if line.starts_with("[") => stack.push(line.to_string().to_lowercase()),
        line if line.starts_with("<") => stack.push(line.to_string()),
        _ => parse_opcode(stack, trimmed_line), // Handle other cases or ignore them
    }
}

/// Parses a given input string to extract the contents of a macro definition 
/// and determine the line number where the macro definition starts.
///
/// # Arguments
///
/// * `contents` - The input string containing code that may include a macro definition.
///
/// # Returns
///
/// A tuple containing:
/// - A string representing the contents of the macro.
/// - The line number where the macro definition starts.
///
fn parse_macro(contents: String) -> (String, usize) {
    let mut macro_contents = String::new();
    let mut start = 0; // line number where macro starts
    let mut in_macro = false;

    for (line_number, line) in contents.lines().enumerate() {
        // in macro
        if in_macro && !line.starts_with("}") {
            // add line to macro_contents with a new line
            macro_contents.push_str(line);
            macro_contents.push_str("\n");
        }

        // start of macro
        if !in_macro && line.starts_with("#define macro") {
            start = line_number;
            in_macro = true;
        }

        // end of macro
        if in_macro && line.starts_with("}") {
            in_macro = false;
        }
    }

    return (macro_contents, start);
}

fn replace_macro(start: usize, comments: String, contents: String) -> String {
    // put lines into a vector
    let mut content_lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();

    let mut ii = 0;
    //replace
    for index in start + 1..=comment_lines.len() + 1 {
        content_lines[index] = comment_lines[ii].clone();
        ii += 1;
    }

    // content lines to string with new line
    let mut final_text = String::new();
    for line in content_lines {
        final_text.push_str(line.as_str());
        final_text.push_str("\n");
    }

    println!("{}", final_text);

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
            parse_line(&mut stack, l.to_string());
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
