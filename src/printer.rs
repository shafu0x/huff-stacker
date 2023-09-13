use crate::stack::Stack;

pub struct Printer {
    macro_content: String,
    stack: Stack,
    longest_line: usize,
}

impl Printer {
    pub fn new(macro_content: String, stack: Stack, longest_line: usize) -> Printer {
        Printer {
            macro_content,
            stack,
            longest_line,
        }
    }

    pub fn print(&self) {
        let mut final_text = String::new();
        for (i, line) in self.macro_content.lines().enumerate() {
            let final_len = self.longest_line - line.len() + 1;
            final_text.push_str(line);
            for _ in 0..final_len {
                final_text.push_str(" ");
            }
            final_text.push_str(" // ");
            final_text.push_str("[");
            final_text.push_str(
                self.stack.values[i]
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
    }
}
