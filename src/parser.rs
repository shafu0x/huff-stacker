use regex::Regex;
use std::fs::File;
use std::io::Read;

use crate::function::Function;
use crate::opcodes::*;
use crate::printer::Printer;
use crate::stack::Stack;

// We insert this placeholder into a function if it takes more than 0 arguments.
// The stack usese this placeholder to determine where to insert the arguments.
const TAKES_PLACEHOLDER: &str = "$takes$";

pub struct Parser {
    functions: Vec<Function>,
    contents: String,
}

fn generate_stack(function: &mut Function) -> Stack {
    let mut stack = Stack::new();
    for line in function.body.lines() {
        parse_line(&mut stack, line.to_string(), function.takes);
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
    let mut function = Function::new();

    let mut skip = 0;
    if last_start > 0 {
        skip = last_start + 1;
    }

    for (line_number, line) in contents.lines().skip(skip).enumerate() {
        // in macro
        if in_macro && !line.trim().starts_with("}") {
            // add line to body with a new line
            body.push_str(line);
            body.push_str("\n");
        }

        // start of macro
        if !in_macro && line.trim().starts_with("#define macro") {
            start = line_number + skip;
            function.start = start;
            function.takes = parse_takes(line);
            in_macro = true;
        }

        // end of macro
        if in_macro && line.trim().starts_with("}") {
            in_macro = false;
            // if the function takes arguments, we need to insert a placeholder
            if function.takes > 0 {
                body = format!("{}\n{}", TAKES_PLACEHOLDER, body);
            }
            function.body = body;
            function.stack = generate_stack(&mut function);
            return Some(function);
        }
    }

    None::<Function>
}

// get the number of arguments a function takes
fn parse_takes(line: &str) -> i32 {
    let re = Regex::new(r"takes \((\d+)\)").unwrap();

    if let Some(captures) = re.captures(line) {
        if let Some(value_str) = captures.get(1) {
            if let Ok(value) = value_str.as_str().parse::<i32>() {
                return value;
            }
        }
    }
    0
}

/// This function takes a mutable reference to a `Stack` and a `line` as input. It trims the
/// `line`, checks its content, and pushes the result onto the `Stack` or delegates to `parse_opcode`
/// for further processing if none of the specific cases match.
fn parse_line(stack: &mut Stack, line: String, takes: i32) {
    let trimmed_line = line.trim();

    match trimmed_line {
        line if line.starts_with(TAKES_PLACEHOLDER) => stack.push_takes(takes), 
        line if line.starts_with("0x") => stack.push(line.to_string()), // constant
        line if line.starts_with("[") => stack.push(line.to_lowercase()), // reference
        line if line.starts_with("<") => stack.push(line.to_string()),
        line if line.starts_with("//") => stack.dup_last(), // comment
        _ => stack.update(Opcode::from_string(trimmed_line)),
    }
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
