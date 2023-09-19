use regex::Regex;
use std::fs::File;
use std::io::Read;

use crate::function::Function;
use crate::token::Token;

const MACRO_START: &str = "#define macro";
const MACRO_END: &str = "}";

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
/// * `skip` - The number of lines to skip before parsing the `contents`.
///
/// # Returns
///
/// An `Option` containing the parsed `Function` if found, or `None` if no valid
/// function is present in the `contents`.
fn parse_function(contents: &str, skip: usize) -> Option<Function> {
    let mut function = Function::new();
    let mut in_function = false;

    for (line_number, line) in contents.lines().skip(skip).enumerate() {
        let trimmed_line = line.trim();

        // start of function
        if trimmed_line.starts_with(MACRO_START) {
            function.start = line_number + skip;
            function.name = parse_function_name(line);
            let (takes, returns) = parse_function_args(line);
            function.takes = takes;
            function.returns = returns;
            in_function = true;
            continue;
        }

        // end of function
        if trimmed_line.starts_with(MACRO_END) {
            function.end = line_number + skip;
            function.gen_stack_history();
            in_function = false;
            return Some(function);
        }

        // inside function
        if in_function {
            function.body.push_str(&format!("{line}\n"));
            continue;
        }
    }
    None
}

// return the first match for the regex found in the line
fn parse_regex(line: &str, regex: &Regex) -> Option<String> {
    if let Some(captures) = regex.captures(line) {
        if let Some(value_str) = captures.get(1) {
            return Some(value_str.as_str().to_string());
        }
    }
    None
}

fn parse_function_name(line: &str) -> String {
    let re = Regex::new(r"#define\s+macro\s+(\w+)").unwrap();
    parse_regex(line, &re).unwrap_or_default()
}

fn parse_function_args(line: &str) -> (i32, i32) {
    let takes_re = Regex::new(r"takes \((\d+)\)").unwrap();
    let returns_re = Regex::new(r"returns \((\d+)\)").unwrap();

    let takes = parse_regex(line, &takes_re)
        .unwrap_or_default()
        .parse::<i32>()
        .unwrap_or_default();
    let returns = parse_regex(line, &returns_re)
        .unwrap_or_default()
        .parse::<i32>()
        .unwrap_or_default();

    (takes, returns)
}

pub fn parse(path: &str) -> Vec<Function> {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    let mut functions = Vec::new();
    let mut skip = 0;
    while let Some(function) = parse_function(&contents, skip) {
        skip = function.end + 1;
        functions.push(function);
    }
    functions
}
