


pub enum LexerErrorKind {
    UnknownSymbol(char),
    InvalidMacro(String),
    InvalidDirective(String),
    InvalidNumberLit(String),
    InvalidCharacterInString(char),
    UnterminatedString,
}

pub struct LexerError {
    kind: LexerErrorKind,
    line: i32,
}

impl LexerError {
    pub fn desc(self: &Self) -> String {
        format!("*** LEXER ERROR [LINE {}]: {}", &self.line, self.kind.desc())
    }

    pub fn new(kind: LexerErrorKind, line: i32) -> Self {
        LexerError { kind, line }
    }
}

impl LexerErrorKind {
    pub fn desc(self: &Self) -> String {
        match self {
            Self::UnknownSymbol(c) => format!("Unknown character found: {c}"),
            Self::InvalidMacro(s) => format!("Invalid macro found: {s}"),
            Self::InvalidDirective(s) => format!("Invalid directive found: {s}"),
            Self::InvalidNumberLit(s) => format!("Invalid number literal found: {s}"),
            Self::InvalidCharacterInString(c) => format!("Invalid character in a string found: {c}"),
            Self::UnterminatedString => format!("Unterminated string found")
        }
    }
}