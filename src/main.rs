mod function;
mod opcodes;
mod parser;
mod printer;
mod stack;
mod token;

use parser::Parser;
use printer::Printer2;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_in = args[1].to_string();
    let path_out = args[2].to_string();

    let functions = Parser::parse(&path_in);

    Printer2::write(&path_in, &functions, &path_out);
}
