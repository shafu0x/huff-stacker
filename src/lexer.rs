use std::fs::File;
use std::io::Read;

pub struct Lexer {
    path: String,
}

fn parse_opcode(line: String) {
    let l = line.trim_start().trim_end();

    if l.starts_with("0x") {
        println!("{}", l);
    }
    
    if l == "calldataload" {
        println!("{}", l);
    }

    if l.starts_with("[") {
        println!("{}", l);
    }

    if l == "sstore" {
        println!("{}", l);
    }

}

fn parse_macro(contents: String) -> String{
    let mut macro_contents = String::new();

    let mut in_macro = false;
    for l in contents.lines() {
        // in macro
        if in_macro && !l.starts_with("}") {
            // add line to macro_contents with a new line
            macro_contents.push_str(l);
            macro_contents.push_str("\n");
        }

        // start of macro
        if !in_macro && l.starts_with("#define macro") {
            in_macro = true;
        }

        // end of macro
        if in_macro && l.starts_with("}") {
            in_macro = false;
        }
    }

    macro_contents
}

impl Lexer {
    pub fn new(path: String) -> Lexer {
        Lexer { path: path }
    }

    pub fn read_file(&self) {
        let mut file = File::open(self.path.as_str()).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error reading file");

        // TODO: refactor the clone
        let macro_contents = parse_macro(contents.clone());
        // println!("{}", macro_contents);

        for l in macro_contents.lines() {
            parse_opcode(l.to_string());
        }

    }
}
