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
        for function in self.functions {
            let comments = self.create_comments(function);
            println!("{}", comments);
            self.merge(function, contents.clone(), comments) // refactor clone
        }
    }

    pub fn merge(&self, function: &Function, contents: String, comments: String) {
        let mut content_lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
        let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();

        let mut ii = 0;
        println!("content_lines: {}", function.start);
        //replace
        for index in function.start + 1..=comment_lines.len() + 1 {
            content_lines[index] = comment_lines[ii].clone();
            ii += 1;
        }

        // content lines to string with new line
        let mut final_text = String::new();
        for line in content_lines {
            final_text.push_str(line.as_str());
            final_text.push_str("\n");
        }

        // println!("{}", final_text);
    }
}
