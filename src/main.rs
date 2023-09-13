mod lexer;
mod stack;
mod stack2;

use lexer::Lexer;

const PATH: &str = "/home/shafu/huff-stack-generator/macro.txt";

fn main() {
    let lexer = Lexer::new(PATH.to_string());
    lexer.read_file();
}
