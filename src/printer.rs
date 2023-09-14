use std::fs::File;
use std::io::Write;

use crate::function::Function;
use crate::stack::Stack;

pub struct Printer<'a> {
    function: &'a Function,
}

fn replace_macro(start: usize, comments: String, contents: String) -> String {
    // put lines into a vector
    let mut content_lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    let comment_lines: Vec<String> = comments.lines().map(|l| l.to_string()).collect();

    let mut ii = 0;
    //replace
    for index in start + 1..=comment_lines.len() + 1 {
        content_lines[index] = comment_lines[ii].clone();
        ii += 1;
    }

    // content lines to string with new line
    let mut final_text = String::new();
    for line in content_lines {
        final_text.push_str(line.as_str());
        final_text.push_str("\n");
    }

    println!("{}", final_text);

    return final_text;
}

impl<'a> Printer<'a> {
    pub fn new(function: &'a Function) -> Printer<'a> {
        Printer { function }
    }

    pub fn print(&self) -> String {
        let mut final_text = String::new();
        for (i, line) in self.function.body.lines().enumerate() {
            let final_len = self.function.longest_line - line.len() + 1;
            final_text.push_str(line);
            for _ in 0..final_len {
                final_text.push_str(" ");
            }
            final_text.push_str(" // ");
            final_text.push_str("[");
            final_text.push_str(
                self.function.stack.values[i]
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
        println!("{}", final_text);
        final_text
    }

    pub fn write(&self) {
        let comments = self.print();
        let with_comments =
            replace_macro(self.function.start, comments, self.function.body.clone());

        // write comments to a file
        let mut file = File::create("output.huff").expect("Unable to create file");
        file.write_all(with_comments.as_bytes())
            .expect("Unable to write data");
    }
}
