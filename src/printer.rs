use std::fs::File;
use std::io::Write;
use std::io::Read;

use crate::function::Function;
use crate::opcodes::STOP;

fn is_stop(mut line: String) -> (String, bool) {
    let stop_sign = " -- end";
    if line.contains(STOP.name) {
        line.push_str(stop_sign);
        return (line, true);
    }
    return (line, false);
}

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

// merge the comments with the original file contents
fn merge(
    function: &Function,
    content_lines: &mut Vec<String>,
    comments: String,
) -> Vec<String> {
    let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();

    let mut i = 0;
    if function.takes > 0 {
        i = 1;
    }
    for index in function.start + 1..=function.start + comment_lines.len() {
        let content_line = content_lines[index].trim().clone();
        if !content_line.starts_with("//") {
            let (content_line, is_stop) = is_stop(comment_lines[i].clone());
            content_lines[index] = content_line;

            if is_stop {
                break;
            }
        }
        i += 1;
    }

    content_lines.to_vec()
}

pub fn write(path_in: &str, functions: &Vec<Function>, path_out: &str) {
    let mut file = File::open(path_in).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let mut content_lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    for function in functions {
        let comments = create_comments(function);
        content_lines = merge(function, &mut content_lines, comments) // refactor clone
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
