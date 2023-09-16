mod function;
mod opcodes;
mod parser;
mod printer;
mod stack;

use parser::Parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_in = args[1].to_string();
    let path_out = args[2].to_string();

    let mut parser = Parser::new();
    parser.parse(&path_in);
    parser.write(&path_out);
}
