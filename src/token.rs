use crate::opcodes::{Opcode, UNKNOWN};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Constant, 
    Opcode,
    Takes_Placeholder,
    Reference, 
    Variable,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub opcode: Option<Opcode>, // Only has an opcode if token_type is Opcode
}

impl Token {
    pub fn new() -> Token {
        Token {
            token_type: TokenType::Unknown,
            value: String::new(),
            opcode: Some(UNKNOWN),
        }
    }

    pub fn from_string(word: &str) -> Token {
        let word = word.trim();
        let mut token = Token::new();
        let token_type = match word {
            _ if word.starts_with("0x") => TokenType::Constant,
            _ if word.starts_with("[") => TokenType::Reference,
            _ if word.starts_with("<") => TokenType::Variable,
            "$takes$" => TokenType::Takes_Placeholder,
            _ => TokenType::Opcode,
        };
        
        token.token_type = token_type;
        token.value = word.to_string();

        if token.token_type == TokenType::Opcode {
            token.opcode = Some(Opcode::from_string(word));
        }

        token
    }
}
