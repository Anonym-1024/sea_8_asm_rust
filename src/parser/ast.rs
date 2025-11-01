
use crate::lexer::token;

#[derive(Debug)]
pub enum AstNodeKind {
    Terminal,
    File,
    Statements,
    Statement,
    ResDirective,
    TypeDirective,
    ByteDirective,
    BytesDirective,
    ArrDirective,
    Assignment,
    AssignmentRepetition,
    AssignmentValues,
    AssignmentValue,
    StartDirective,
    LabelDirective,
    Instruction,
    ConditionCode,
    InstructionArguments,
    InstructionArgument,
    LongRegister,
    Macro,
    MacroArguments,
    MacroArgument,
}
#[derive(Debug)]
pub struct AstNode {
    kind: AstNodeKind,
    children: Vec<AstNode>,
    terminal: Option<token::Token>
}

impl AstNode {
    pub fn nonterminal(kind: AstNodeKind, children: Vec<AstNode>) -> AstNode {
        AstNode { kind, children, terminal: None }
    }

    pub fn terminal(terminal: token::Token) -> AstNode {
        AstNode { kind: AstNodeKind::Terminal, children: Vec::new(), terminal: Some(terminal) }
    }

    pub fn kind_desc(&self) -> String {
        match self.kind {
            AstNodeKind::Instruction => "instruction statement.".to_string(),
            AstNodeKind::ResDirective => "reserve directive.".to_string(),
            AstNodeKind::StartDirective => "start directive".to_string(),
            AstNodeKind::LabelDirective => "lebel directive".to_string(),
            AstNodeKind::Macro => "macro directive".to_string(),
            _ => "".to_string()
        }
    }
}