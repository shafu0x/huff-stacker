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

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_in = args[1].to_string();
    let path_out = args[2].to_string();

    let stack_order = "right";

    let functions = parse(&path_in);
    write(&path_in, &path_out, &functions, stack_order);
}
