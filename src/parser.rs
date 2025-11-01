use crate::lexer::token::{self, Token, TokenKind};
use ast::{AstNode, AstNodeKind};
use error::ParserError;
use result::ParserResult;


pub mod ast;
pub mod error;
pub mod result;






pub fn parse(tokens: &[Token]) -> ParserResult<AstNode, ParserError> {

    

    let mut index = 0;
    let mut line = 1;

    return parse_file(tokens, &mut index, &mut line);


    
}





fn pop_token_if_kind(tokens: &[Token], index: &mut usize, line: &mut i32, kind: TokenKind) -> Option<Token> {
    if *index >= tokens.len() {
        return None;
    }
    let token = &tokens[*index];


    if token.kind == kind {
        *line = token.line;
        *index += 1;
        Some(Token::new(kind, String::from(&token.lexeme), *line))
    } else {
        None
    }
}

fn pop_token_if_lexeme(tokens: &[Token], index: &mut usize, line: &mut i32, lexeme: &str) -> Option<Token> {
    if *index >= tokens.len() {
        return None;
    }
    let token = &tokens[*index];


    if token.lexeme == lexeme {
        *line = token.line;
        *index += 1;
        Some(Token { kind: token.kind, lexeme: String::from(lexeme), line: *line })
    } else {
        None
    }
}



fn lookahead<'a>(tokens: &'a [Token], index: usize, k: usize) -> Option<&'a Token> {
    if index + k < tokens.len() { Some(&tokens[index + k]) } else { None }
}



fn parse_file(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    let statements_node = parse_statements(tokens, index, line);
    match statements_node {
        ParserResult::Some(node) => children.push(node),
        ParserResult::Err(error) => return ParserResult::Err(error),
        ParserResult::None => panic!("INVALID"),
    }


    if pop_token_if_kind(tokens, index, line,  TokenKind::Eof).is_some() {
        ParserResult::Some(AstNode::nonterminal(AstNodeKind::File, children))
    } else {

        ParserResult::Err(ParserError::new("Expected end of file", *line))
    }
}


fn parse_statements(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    loop {
        let statement_node = parse_statement(tokens, index, line);

        match statement_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { break; },
            ParserResult::Err(_) => { return statement_node; },
        }
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::Statements, children))
}


fn parse_statement(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    while pop_token_if_lexeme(tokens, index, line, "\n").is_some() { }


    let child_node = parse_res_directive(tokens, index, line)
        .or(|| parse_start_directive(tokens, index, line))
        .or(|| parse_label_directive(tokens, index, line))
        .or(|| parse_instruction(tokens, index, line))
        .or(|| parse_macro(tokens, index, line));

    let mut child_node_desc = String::new();
    
    match child_node {
        ParserResult::Some(node) => { 
            child_node_desc.push_str(node.kind_desc().as_str());
            children.push(node); 
        },
        ParserResult::None => { return child_node; },
        ParserResult::Err(_) => { return child_node; },
    }
    
    let new_line_token = pop_token_if_lexeme(tokens, index, line, "\n");
    
    if new_line_token.is_none() && let Some(t) = lookahead(tokens, *index, 0) && t.kind != TokenKind::Eof {
        return ParserResult::Err(ParserError::new(format!("Expected a new line after a statement, not {} ({} unterminated).", t.lexeme, child_node_desc).as_str(), *line));
    }



    ParserResult::Some(AstNode::nonterminal(AstNodeKind::Statement, children))

}



fn parse_res_directive(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_lexeme(tokens, index, line,".res") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Identifier) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Expected identifier after .res directive.", *line))
    }

    let type_directive_node = parse_type_directive(tokens, index, line);
    match type_directive_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { return ParserResult::Err(ParserError::new("Expected type in .res directive.", *line)); },
        ParserResult::Err(_) => { return type_directive_node; },
    }

    let assignment_node = parse_assignment(tokens, index, line);
    match assignment_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => {  },
        ParserResult::Err(_) => { return assignment_node; },
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::ResDirective, children))

    
}


fn parse_type_directive(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    let child_node = parse_byte_directive(tokens, index, line)
        .or(|| parse_bytes_directive(tokens, index, line))
        .or(|| parse_arr_directive(tokens, index, line));


    match child_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { return child_node; },
        ParserResult::Err(_) => { return child_node; },
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::TypeDirective, children))

}


fn parse_byte_directive(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    if let Some(token) = pop_token_if_lexeme(tokens, index, line, ".byte") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::ByteDirective, children))
}


fn parse_bytes_directive(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    if let Some(token) = pop_token_if_lexeme(tokens, index, line, ".bytes") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Number) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Expected a number (element size) after a .bytes directive", *line));
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::BytesDirective, children))
}



fn parse_arr_directive(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, ".arr") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Number) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Expected a number (array size) after an .arr directive", *line));
    }

    let type_directive_node = parse_type_directive(tokens, index, line);
    match type_directive_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { return ParserResult::Err(ParserError::new("Expected type in .arr directive.", *line)); },
        ParserResult::Err(_) => { return type_directive_node; },
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::ArrDirective, children)) 

}



fn parse_assignment(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, "{") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    let assignment_values_node = parse_assignment_values(tokens, index, line);
    match assignment_values_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { panic!("NEVER") },
        ParserResult::Err(_) => { return assignment_values_node; },
    }

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, "}") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Assignment list must be terminated with }.", *line));
    }


    let assignment_repetition_node = parse_assignment_repetition(tokens, index, line);
    match assignment_repetition_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => {  },
        ParserResult::Err(_) => { return assignment_repetition_node; },
    }


    ParserResult::Some(AstNode::nonterminal(AstNodeKind::Assignment, children)) 

}



fn parse_assignment_repetition(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, "*") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Number) {
        children.push(AstNode::terminal(token));
    } 

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::AssignmentRepetition, children)) 

}


fn parse_assignment_values(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    loop {
        let assignment_value_node = parse_assignment_value(tokens, index, line);
        match assignment_value_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { break; },
            ParserResult::Err(_) => { return assignment_value_node; },
        }
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::AssignmentValues, children)) 

}


fn parse_assignment_value(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    let assignment_node = parse_assignment(tokens, index, line);
    match assignment_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => {  },
        ParserResult::Err(_) => { return assignment_node; },
    }
    
    if !children.is_empty() {
        return ParserResult::Some(AstNode::nonterminal(AstNodeKind::AssignmentValue, children))
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Number) {
        children.push(AstNode::terminal(token));
    } else if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::String) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None
    }
    
    ParserResult::Some(AstNode::nonterminal(AstNodeKind::AssignmentValue, children)) 


}


fn parse_start_directive(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, ".start") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, ":") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Expected : after a .start directive.", *line))
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::StartDirective, children)) 

}


fn parse_label_directive(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Identifier) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, ":") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Expected : after a label directive.", *line))
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::LabelDirective, children)) 
}



fn parse_instruction(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Instruction) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }


    let condition_code_node = parse_condition_code(tokens, index, line);
    match condition_code_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => {  },
        ParserResult::Err(_) => { return condition_code_node; },
    }


    let arguments_node = parse_instruction_arguments(tokens, index, line);
    match arguments_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { panic!("NEVER") },
        ParserResult::Err(_) => { return arguments_node; },
    }


    ParserResult::Some(AstNode::nonterminal(AstNodeKind::Instruction, children)) 

    
}


fn parse_condition_code(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, ":") {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::ConditionCode) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Expected condition code after : in an instruction.", *line));
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::ConditionCode, children)) 

}


fn parse_instruction_arguments(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    loop {
        let instruction_argument_node = parse_instruction_argument(tokens, index, line);
        match instruction_argument_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { break; },
            ParserResult::Err(_) => { return instruction_argument_node; },
        }
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::InstructionArguments, children)) 

}


fn parse_instruction_argument(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    let long_register_node = parse_long_register(tokens, index, line);
    match long_register_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { },
        ParserResult::Err(_) => { return long_register_node; },
    }

    if !children.is_empty() {
        return ParserResult::Some(AstNode::nonterminal(AstNodeKind::InstructionArgument, children)); 
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Register) {
        children.push(AstNode::terminal(token));
    } else if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::SystemRegister) {
        children.push(AstNode::terminal(token));
    } else if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Port) {
        children.push(AstNode::terminal(token));
    } else if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Number) {
        children.push(AstNode::terminal(token));
    } else if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::String) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::InstructionArgument, children))

}


fn parse_long_register(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();

    if lookahead(tokens, *index, 1).is_none_or(|t| t.lexeme != "~")  {
        return ParserResult::None;
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Register) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::Err(ParserError::new("Expected a register before ~ in a long register argument.", *line));
    }

    if let Some(token) = pop_token_if_lexeme(tokens, index, line, "~") {
        children.push(AstNode::terminal(token));
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::LongRegister, children))

}


fn parse_macro(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Macro) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }


    let condition_code_node = parse_condition_code(tokens, index, line);
    match condition_code_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => {  },
        ParserResult::Err(_) => { return condition_code_node; },
    }


    let arguments_node = parse_macro_arguments(tokens, index, line);
    match arguments_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { panic!("NEVER") },
        ParserResult::Err(_) => { return arguments_node; },
    }


    ParserResult::Some(AstNode::nonterminal(AstNodeKind::Macro, children)) 

}


fn parse_macro_arguments(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    loop {
        let macro_argument_node = parse_macro_argument(tokens, index, line);
        match macro_argument_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { break; },
            ParserResult::Err(_) => { return macro_argument_node; },
        }
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::MacroArguments, children)) 

}


fn parse_macro_argument(tokens: &[Token], index: &mut usize, line: &mut i32) -> ParserResult<AstNode, ParserError> {
    let mut children: Vec<AstNode> = Vec::new();


    let long_register_node = parse_long_register(tokens, index, line);
    match long_register_node {
        ParserResult::Some(node) => { children.push(node); },
        ParserResult::None => { },
        ParserResult::Err(_) => { return long_register_node; },
    }

    if !children.is_empty() {
        return ParserResult::Some(AstNode::nonterminal(AstNodeKind::InstructionArgument, children)); 
    }

    if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Register) {
        children.push(AstNode::terminal(token));
    } else if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Number) {
        children.push(AstNode::terminal(token));
    } else if let Some(token) = pop_token_if_kind(tokens, index, line, TokenKind::Identifier) {
        children.push(AstNode::terminal(token));
    } else {
        return ParserResult::None;
    }

    ParserResult::Some(AstNode::nonterminal(AstNodeKind::MacroArgument, children))
}