use crate::stack::Stack;
use crate::function::Function;

pub struct Printer<'a> {
    function: &'a Function
}

impl<'a> Printer<'a> {
    pub fn new(function: &'a Function) -> Printer<'a> {
        Printer {
            function
        }
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
}
