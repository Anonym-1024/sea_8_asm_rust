
#[derive(Debug)]
pub enum TokenKind {
    TokenInstruction,
    TokenMacro,
    TokenDirective,
    TokenRegister,
    TokenPort,
    TokenSystemRegister,
    TokenConditionCode,
    TokenIdentifier,
    TokenNumber,
    TokenString,
    TokenPunctuation,
    TokenEof
}

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: i32) -> Self {
        Self  {kind: kind, lexeme: lexeme, line: line}
    }

    pub fn eof_token(line: i32) -> Self {
        Self { kind: TokenKind::TokenEof, lexeme: String::new(), line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {}, type: {:?}, lexeme: {}", self.line, self.kind, self.lexeme)
    }
}