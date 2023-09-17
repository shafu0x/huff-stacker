enum TokenType {
    Constant, 
    Opcode,
    Takes_Placeholder,
    Reference, 
    Variable,
    Comment, 
}

pub struct Token {
    pub name: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
    pub token_type: TokenType,
}
