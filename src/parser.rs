use regex::Regex;
use std::fs::File;
use std::io::Read;

use crate::function::Function;
use crate::opcodes::*;
// use crate::printer::Printer;
use crate::stack::{Stack, TAKES_PLACEHOLDER};
use crate::token::Token;

const MACRO_START: &str = "#define macro";
const MACRO_END: &str = "}";

pub struct Parser {
    functions: Vec<Function>,
    contents: String,
}

pub fn parse_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for word in line.split_whitespace() {
        tokens.push(Token::from_string(word));
    }
    tokens
}

/// Parses a given input string to extract the contents of a macro definition
/// and determine the line number where the macro definition starts.
///
/// # Arguments
///
/// * `contents` - The input string containing code that may include a macro definition.
/// * `last_start` - The line number where the last macro definition started.
///
/// # Returns
///
/// An `Option` containing the parsed `Function` if found, or `None` if no valid
/// function is present in the `contents`.
fn parse_function(contents: String, skip: usize) -> Option<Function> {
    let mut function = Function::new();
    let mut in_function = false;

    for (line_number, line) in contents.lines().skip(skip).enumerate() {
        // in function
        if in_function && !line.trim().starts_with("}") {
            function.body.push_str(line);
            function.body.push_str("\n");
            continue;
        }

        // start of function
        if !in_function && line.trim().starts_with(MACRO_START) {
            function.start = line_number + skip;
            function.takes = parse_takes(line);
            in_function = true;
            continue;
        }

        // end of function
        if in_function && line.trim().starts_with(MACRO_END) {
            in_function = false;
            // if the function takes arguments, we need to insert a placeholder
            if function.takes > 0 {
                function.body = format!("{}\n{}", TAKES_PLACEHOLDER, function.body);
            }
            function.gen_stack_history();
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

        let mut skip = 0;
        while let Some(function) = parse_function(contents.clone(), skip) {
            skip = function.start;
            // tbh I don't fully understand why we need this. lol
            if skip > 0 {
                skip += 1;
            }

            self.functions.push(function);
        }
    }

    // pub fn write(&self, path: &str) {
    //     Printer::new(&self.functions).write(self.contents.clone(), path);
    // }
}
