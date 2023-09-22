use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::function::Function;
use crate::opcodes::STOP;

const END_SIGN: &str = " -- end";
const COMMENT_START: &str = "//";

fn create_comments(function: &Function) -> String {
    let mut final_text = String::new();
    for (i, line) in function.body.lines().enumerate() {
        let final_len = function.longest_line() - line.len() + 1;
        final_text.push_str(line);
        for _ in 0..final_len {
            final_text.push_str(" ");
        }
        final_text.push_str(" // ");
        final_text.push_str("[");
        final_text.push_str(
            function.stack_history.stacks[i]
                .values
                .iter()
                .map(|token| token.to_str())
                .collect::<Vec<_>>()
                .join(", ")
                .as_str(),
        );
        final_text.push_str("]");
        final_text.push_str("\n");
    }
    final_text
}

fn is_comment_or_empty(line: &str) -> bool {
    line.trim().starts_with(COMMENT_START) || line.trim().is_empty()
}

// merge the comments with the original file contents
fn merge(function: &Function, content_lines: &mut Vec<String>, comments: String) -> Vec<String> {
    let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();

    for (i, index) in (function.start + 1..=function.start + comment_lines.len()).enumerate() {
        if let Some(content_line) = content_lines.get_mut(index) {
            let comment_line = &comment_lines[i];
            if !is_comment_or_empty(comment_line) {
                *content_line = comment_line.clone();
                if comment_line.contains(STOP.name) {
                    content_line.push_str(END_SIGN);
                    break;
                }
            }
        }
    }

    content_lines.to_vec()
}

pub fn write(path_in: &str, path_out: &str, functions: &Vec<Function>) {
    let mut file = File::open(path_in).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let mut content_lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    for function in functions {
        let comments = create_comments(function);
        content_lines = merge(function, &mut content_lines, comments);
    }

    let mut final_text = String::new();
    for line in content_lines {
        final_text.push_str(line.as_str());
        final_text.push_str("\n");
    }

    let mut file = File::create(path_out).expect("Error creating file");
    file.write_all(final_text.as_bytes())
        .expect("Error writing file");
}
