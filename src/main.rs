mod function;
mod lexer;
mod opcodes;
mod printer;
mod stack;

use lexer::Lexer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].to_string();
    let mut lexer = Lexer::new(path);
    lexer.parse();
    lexer.write();
}
