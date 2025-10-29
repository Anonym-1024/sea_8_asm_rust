
use crate::lexer::token;


enum AstNodeKind {
    NodeTerminal,
    NodeFile,
    NodeStatements,
    NodeStatement,
    NodeMainDierctive
}

struct AstNode {
    kind: AstNodeKind,
    children: Vec<AstNode>,
    terminal: Option<token::Token>
}

impl AstNode {
    fn nonterminal(kind: AstNodeKind, children: Vec<AstNode>) -> AstNode {
        AstNode { kind, children, terminal: None }
    }

    fn terminal(terminal: token::Token) -> AstNode {
        AstNode { kind: AstNodeKind::NodeTerminal, children: Vec::new(), terminal: Some(terminal) }
    }
}