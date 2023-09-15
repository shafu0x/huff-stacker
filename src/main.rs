mod function;
mod opcodes;
mod parser;
mod printer;
mod stack;

use parser::Parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].to_string();
    let mut parser = Parser::new(path);
    parser.parse();
    parser.write();
}
