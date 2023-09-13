mod lexer;
mod printer;
mod stack;

use std::env;
use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lexer = Lexer::new(args[1].to_string());
    lexer.read_file();
}
