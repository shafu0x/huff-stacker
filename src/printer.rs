use std::fs::File;
use std::io::Write;

use crate::function::Function;
use crate::stack::Stack;

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
            let final_len = function.longest_line - line.len() + 1;
            final_text.push_str(line);
            for _ in 0..final_len {
                final_text.push_str(" ");
            }
            final_text.push_str(" // ");
            final_text.push_str("[");
            final_text.push_str(
                function.stack.values[i]
                    .iter()
                    .rev()
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

    pub fn write(&self, contents: String) {
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

        // println!("{}", final_text);
        let mut file = File::create("macro.txt").expect("Error creating file");
        file.write_all(final_text.as_bytes())
            .expect("Error writing file");
    }

    pub fn merge(
        &self,
        function: &Function,
        content_lines: &mut Vec<String>,
        comments: String,
    ) -> Vec<String> {
        let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();

        let mut ii = 0;
        for index in function.start + 1..=function.start+comment_lines.len() {
            content_lines[index] = comment_lines[ii].clone();
            ii += 1;
        }

        content_lines.to_vec()

    }
}
