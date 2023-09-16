use std::fs::File;
use std::io::Write;

use crate::function::Function;
use crate::opcodes::STOP;

pub struct Printer<'a> {
    functions: &'a Vec<Function>,
}

impl<'a> Printer<'a> {
    pub fn new(functions: &'a Vec<Function>) -> Printer<'a> {
        Printer { functions }
    }

    pub fn create_comments(&self, function: &Function) -> String {
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
                function.stack.values[i]
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
                    .as_str(),
            );
            final_text.push_str("]");
            final_text.push_str("\n");
        }
        final_text
    }

    pub fn write(&self, contents: String, path: &str) {
        let mut content_lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
        for function in self.functions {
            let comments = self.create_comments(function);
            content_lines = self.merge(function, &mut content_lines, comments) // refactor clone
        }

        let mut final_text = String::new();
        for line in content_lines {
            final_text.push_str(line.as_str());
            final_text.push_str("\n");
        }

        let mut file = File::create(path).expect("Error creating file");
        file.write_all(final_text.as_bytes())
            .expect("Error writing file");
    }

    // merge the comments with the original file contents
    pub fn merge(
        &self,
        function: &Function,
        content_lines: &mut Vec<String>,
        comments: String,
    ) -> Vec<String> {
        let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();

        let mut i = 0;
        if function.takes > 0 {
            i = 1;
        }
        for index in function.start+1..=function.start + comment_lines.len() {
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
}

fn is_stop(mut line: String) -> (String, bool) {
    let stop_sign = " -- end";
    if line.contains(STOP.name) {
        line.push_str(stop_sign);
        return (line, true);
    }
    return (line, false);
}
