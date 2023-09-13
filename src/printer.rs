use crate::stack::Stack;

pub struct Printer {
    macro_content: String,
    stack: Stack,
}

impl Printer {
    pub fn new(macro_content: String, stack: Stack) -> Printer {
        Printer {
            macro_content,
            stack,
        }
    }

    pub fn print(&self) {
        let mut final_text = String::new();
        for (i, line) in self.macro_content.lines().enumerate() {
            // println!("{}: {}", i, line);

            final_text.push_str(line);
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
