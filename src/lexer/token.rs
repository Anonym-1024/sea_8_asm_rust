
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum TokenKind {
    Instruction,
    Macro,
    Directive,
    Register,
    Port,
    SystemRegister,
    ConditionCode,
    Identifier,
    Number,
    String,
    Punctuation,
    Eof
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: i32) -> Self {
        Self  {kind: kind, lexeme: lexeme, line: line}
    }

    pub fn eof_token(line: i32) -> Self {
        Self { kind: TokenKind::Eof, lexeme: String::new(), line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {}, type: {:?}, lexeme: {}", self.line, self.kind, self.lexeme)
    }
}


