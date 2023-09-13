use std::fs::File;
use std::io::Read;

pub struct Lexer {
    path: String,
}

impl Lexer {
    pub fn new(path: String) -> Lexer {
        Lexer { path: path }
    }

    pub fn read_file(&self) -> String {
        let mut file = File::open(self.path.as_str()).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error reading file");

        let mut in_macro = false;
        for l in contents.lines() {

            // in macro
            if in_macro && !l.starts_with("}") {
                println!("In {}", l);
            }
            
            // start of macro
            if !in_macro && l.starts_with("#define macro") {
                println!("Start {}", l);
                in_macro = true;
            }

            // end of macro
            if in_macro && l.starts_with("}") {
                println!("End {}", l);
                in_macro = false;
            }
        }
        contents
    }
}
