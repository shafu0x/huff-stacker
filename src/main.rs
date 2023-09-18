mod function;
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
    //let path_in = args[1].to_string();
    //let path_out = args[2].to_string();

    let path_in = "/home/shafu/huff-stack-generator/macro.huff";
    let path_out = "/home/shafu/huff-stack-generator/out.huff";

    let functions = parse(&path_in);
    write(&path_in, &functions, &path_out);
}
