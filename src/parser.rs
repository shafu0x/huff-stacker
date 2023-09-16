use regex::Regex;
use std::fs::File;
use std::io::Read;

use crate::function::Function;
use crate::opcodes::*;
use crate::printer::Printer;
use crate::stack::Stack;

pub struct Parser {
    functions: Vec<Function>,
    contents: String,
}

fn generate_stack(function: &Function) -> Stack {
    let mut stack = Stack::new();
    for line in function.body.lines() {
        parse_line(&mut stack, line.to_string());
    }
    stack
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
fn parse_function(contents: String, last_start: usize) -> Option<Function> {
    let mut body = String::new();
    let mut start = 0; // line number where macro starts
    let mut in_macro = false;

    let mut found_function = false;

    let mut function = Function::new();

    let mut skip = 0;
    if last_start > 0 {
        skip = last_start + 1;
    }

    for (line_number, line) in contents.lines().skip(skip).enumerate() {
        // in macro
        if in_macro && !line.starts_with("}") {
            // add line to body with a new line
            body.push_str(line);
            body.push_str("\n");
        }

        // start of macro
        if !in_macro && line.starts_with("#define macro") {
            start = line_number + skip;
            function.start = start;
            function.takes = get_takes(line);
            found_function = true;
            in_macro = true;
        }

        // end of macro
        if in_macro && line.starts_with("}") {
            in_macro = false;
            break;
        }
    }

    if found_function {
        function.body = body;
        function.stack = generate_stack(&function);
        Some(function)
    } else {
        None::<Function>
    }
}

fn get_takes(line: &str) -> i32 {
    let re = Regex::new(r"takes \((\d+)\)").unwrap();

    if let Some(captures) = re.captures(line) {
        if let Some(value_str) = captures.get(1) {
            if let Ok(value) = value_str.as_str().parse::<i32>() {
                return value;
            }
        }
    }

    println!("Failed to parse value as an integer.");
    0
}

/// This function takes a mutable reference to a `Stack` and a `line` as input. It trims the
/// `line`, checks its content, and pushes the result onto the `Stack` or delegates to `parse_opcode`
/// for further processing if none of the specific cases match.
fn parse_line(stack: &mut Stack, line: String) {
    let trimmed_line = line.trim();

    match trimmed_line {
        line if line.starts_with("0x") => stack.push(line.to_string()), // constant
        line if line.starts_with("[") => stack.push(line.to_lowercase()), // reference
        line if line.starts_with("<") => stack.push(line.to_string()),
        line if line.starts_with("//") => stack.dup_last(), // comment
        _ => parse_opcode(stack, trimmed_line),
    }
}

fn parse_opcode(stack: &mut Stack, line: &str) {
    stack.update(Opcode::from_string(line));
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            functions: Vec::new(),
            contents: String::new(),
        }
    }

    pub fn parse(&mut self, path: &str) {
        let mut file = File::open(path).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error reading file");
        self.contents = contents.clone();

        let mut last_start = 0;
        while let Some(function) = parse_function(contents.clone(), last_start) {
            last_start = function.start;

            self.functions.push(function);
        }
    }

    pub fn write(&self, path: &str) {
        Printer::new(&self.functions).write(self.contents.clone(), path);
    }
}
