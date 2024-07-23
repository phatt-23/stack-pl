use std::{ self, fs, io::{self}, path::PathBuf };
use std::collections::HashMap;

use crate::lexer;
use crate::operation::{Operation, OperationKind};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
struct Macro {
    pub loc: crate::location::Location,
    pub name: String,
    pub tokens: Vec<Token>,
}

pub fn compile_tokens_to_operations(tokens: &Vec<Token>) -> Result<Vec<Operation>, io::Error> {
    let mut tokens = tokens.clone();
    tokens.reverse();
    
    let mut operations: Vec<Operation> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut macros: HashMap<String, Macro> = HashMap::new();
    let mut address_counter: usize = 0;

    let mut ip: usize = 0;
    while tokens.len() > 0 {
        let token = tokens.pop().unwrap();

        // compile token to operation
        let mut op = match &token.kind {
            TokenKind::Integer(value) => {
                Operation::new(OperationKind::PushInt(value.clone()), token.loc)
            }
            TokenKind::String(value) => {
                Operation::new(OperationKind::PushStr(value.clone()), token.loc)
            }
            TokenKind::Word(value) => {
                if let Some(op_kind) = OperationKind::from_str(value) {
                    Operation::new(op_kind, token.loc)
                } else {
                    if let Some(m) = macros.get_mut(value) {
                        for i in 0..m.tokens.len() {
                            let t = m.tokens.get(i).unwrap();
                            tokens.push(t.clone()); 
                        }
                        continue;   
                    }
                    panic!("[ERROR]: Unknown word, identifier {} {:?}", token.loc, token.kind);
                }
            }
        };

        // crossreference block operations and handle preprocessor
        match &mut op.kind {
            OperationKind::If(_) => stack.push(ip),
            OperationKind::While => stack.push(ip),
            OperationKind::Do(do_jump) => {
                let while_ip = stack.pop().unwrap();
                assert!(operations[while_ip].kind == OperationKind::While);
                *do_jump = while_ip as i32;
                stack.push(ip);
            }
            OperationKind::Else(_) => {
                let if_ip = stack.pop().unwrap();
                assert!(matches!(operations[if_ip].kind, OperationKind::If(_)));
                if let OperationKind::If(ref mut if_jump) = operations[if_ip].kind {
                    *if_jump = (ip + 1) as i32; 
                }
                stack.push(ip);
            }
            OperationKind::End(end_jump) => {
                let prev_ip = stack.pop().unwrap();
                match &mut operations[prev_ip].kind {
                    OperationKind::If(if_jump) => *if_jump = ip as i32,
                    OperationKind::Else(else_jump) => *else_jump = ip as i32,
                    OperationKind::Do(do_jump) => {
                        *end_jump = *do_jump as i32;
                        *do_jump = (ip + 1) as i32;
                    }
                    _ => panic!("[ERROR] `end` keyword found with no preceding block operations")
                }
            }
            OperationKind::Macro => {
                assert!(tokens.len() > 0, "macro must have a body and be closed by `end`");
                let name = match tokens.pop().unwrap().kind {
                    TokenKind::Word(ref v) => {
                        if let Some(m) = macros.get(v) {
                            panic!("[ERROR]: {ol} Macro with identifier {v:?} already implemented at {ml} (can't use the same identifier for macros)", ol=&op.loc, ml=&m.loc);
                        }
                        if let Some(op_type) = OperationKind::from_str(&v) {
                            panic!("[ERROR]: {} Can't use a builtin keyword {v:?} standing for {op_type:?} as an identifier for `macro`", &op.loc);
                        }
                        v.clone()
                    }
                    TokenKind::Integer(v) => panic!("[ERROR]: {} {v:?} - Expected `macro` identifier `Word` but found `Integer`", &op.loc),
                    TokenKind::String(v) => panic!("[ERROR]: {} {v:?} - Expected `macro` identifier `Word` but found `String`", &op.loc),
                };
                macros.insert(name.clone(), Macro { loc: op.loc.clone(), name: name.clone(), tokens: Vec::new() });
                while let Some(tok) = tokens.pop() {
                    if matches!(&tok.kind, TokenKind::Word(v) if v == "end") {
                        tokens.push(tok);
                        break;
                    }
                    macros.get_mut(&name).unwrap().tokens.insert(0, tok.clone());
                }
                let end = tokens.pop().unwrap_or_else(|| panic!("[ERROR]: {} Macro {name:?} No `end` was found for `macro` block", &op.loc));
                assert!(matches!(&end.kind, TokenKind::Word(v) if v == "end"));
            }
            OperationKind::Include(_) => {
                let include_token = tokens.pop().unwrap();
                match &include_token.kind {
                    TokenKind::String(include_file) => {
                        let src_path = PathBuf::from(&op.loc.file);
                        let inc_path = PathBuf::from(&include_file);
                        
                        let src_dir = src_path.parent().unwrap().to_str().unwrap();
                        let inc_dir = inc_path.parent().unwrap().to_str().unwrap();
                        let inc_file_name = inc_path.file_name().unwrap().to_str().unwrap();

                        let search_dir = [src_dir, "/", inc_dir].concat();

                        let entries = fs::read_dir(&search_dir)
                            .unwrap_or_else(|e| panic!("[ERROR]: {} Directory {inc_dir:?} does not exist: {e}", &include_token.loc));

                        let inc_file_path = entries
                            .filter_map(|e| e.ok())
                            .find(|e| e.file_name().to_str().unwrap() == inc_file_name)
                            .map(|e| ["./", &search_dir.as_str(), "/", e.file_name().to_str().unwrap()].concat())
                            .unwrap_or_else(|| panic!("[ERROR]: {} File {inc_file_name:?} not found in directory {search_dir:?}", &op.loc));
                            
                        match lexer::lex_file_to_tokens(&inc_file_path) {
                            Ok(ref mut include_tokens) => {
                                include_tokens.reverse();
                                tokens.append(include_tokens);
                            }
                            Err(e) => panic!("[ERROR]: {} Could not include file {include_file:?}, Error: {e}", &include_token.loc),
                        }
                    }
                    TokenKind::Integer(v) => panic!("[ERROR]: {} {v:?} - File after `include` must be `String` but found `Integer`", &op.loc),
                    TokenKind::Word(v) => panic!("[ERROR]: {} {v:?} - File after `include` must be `String` but found `Word`", &op.loc),
                }
            }
            _ => {}
        }
        
        // ignore preprocessor operations
        if !matches!(op.kind, OperationKind::Macro | OperationKind::Include(_)) {
            op.address = address_counter;
            operations.push(op);
            address_counter += 1;
        }

        ip += 1;
    }
    
    
    Ok(operations)
}

