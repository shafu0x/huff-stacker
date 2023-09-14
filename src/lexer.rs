use std::fs::File;
use std::io::Read;

use crate::function::Function;
use crate::opcodes::*;
use crate::printer::Printer;
use crate::stack::Stack;

pub struct Lexer {
    path: String,
    functions: Vec<Function>,
    contents: String,
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
fn get_function(contents: String, last_start: usize) -> Option<(String, usize)> {
    let mut macro_lines = String::new();
    let mut start = 0; // line number where macro starts
    let mut in_macro = false;

    let mut found_function = false;

    let mut skip = 0;
    if last_start > 0 {
        skip = last_start + 1;
    }

    for (line_number, line) in contents.lines().skip(skip).enumerate() {
        // in macro
        if in_macro && !line.starts_with("}") {
            // add line to macro_lines with a new line
            macro_lines.push_str(line);
            macro_lines.push_str("\n");
        }

        // start of macro
        if !in_macro && line.starts_with("#define macro") {
            start = line_number + skip;
            in_macro = true;
            found_function = true;
        }

        // end of macro
        if in_macro && line.starts_with("}") {
            in_macro = false;
            break;
        }
    }

    if found_function {
        Some((macro_lines, start))
    } else {
        None::<(String, usize)>
    }
}

/// This function takes a mutable reference to a `Stack` and a `line` as input. It trims the
/// `line`, checks its content, and pushes the result onto the `Stack` or delegates to `parse_opcode`
/// for further processing if none of the specific cases match.
fn parse_line(stack: &mut Stack, line: String) {
    let trimmed_line = line.trim();

    match trimmed_line {
        line if line.starts_with("0x") => stack.push(line.to_string()),
        line if line.starts_with("[") => stack.push(line.to_string().to_lowercase()),
        line if line.starts_with("<") => stack.push(line.to_string()),
        _ => parse_opcode(stack, trimmed_line), // Handle other cases or ignore them
    }
}

fn parse_opcode(stack: &mut Stack, line: &str) {
    stack.update(Opcode::from_string(line));
}

impl Lexer {
    pub fn new(path: String) -> Lexer {
        Lexer {
            path: path,
            functions: Vec::new(),
            contents: String::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut file = File::open(self.path.as_str()).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error reading file");

        let mut last_start = 0;
        let mut i = 0;
        while let Some((function_body, start)) = get_function(contents.clone(), last_start) {
            self.contents = contents.clone();
            last_start = start;

            let mut stack = Stack::new();
            let mut longest_line = 0;

            for line in function_body.lines() {
                if line.len() > longest_line {
                    longest_line = line.len();
                }
                parse_line(&mut stack, line.to_string());
            }

            self.functions
                .push(Function::new(start, function_body, stack, longest_line));

            i += 1;
        }
    }

    pub fn write(&self) {
        Printer::new(&self.functions).write(self.contents.clone());
    }
}
