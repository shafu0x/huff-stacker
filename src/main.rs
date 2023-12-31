mod function;
mod jump;
mod opcodes;
mod parser;
mod printer;
mod stack;
mod token;

use parser::parse;
use printer::write;
use std::env;

// Determine if the stack should be printed from left-to right
// or right-to-left
// Default is left-to-right
fn stack_order(args: Vec<String>) -> &'static str {
    if args.len() == 4 && args[3] == "--right" {
        return "right";
    }
    "left"
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_in = args[1].to_string();
    let path_out = args[2].to_string();

    let functions = parse(&path_in);
    write(&path_in, &path_out, &functions, stack_order(args));
}
