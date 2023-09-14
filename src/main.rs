mod lexer;
mod printer;
mod stack;

use std::env;
use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].to_string();
    let lexer = Lexer::new(path);
    lexer.read_file();
}
