use std::{ self, io, fs, path::PathBuf };
use std::collections::HashMap;

use crate::operation::{Operation, OperationKind};
use crate::token::{Token, TokenKind};
use crate::keyword::KeywordType;
use crate::location::Location;
use crate::lexer;

#[derive(Debug, Clone, PartialEq)]
struct Macro {
    pub loc: Location,
    pub name: String,
    pub tokens: Vec<Token>,
}

impl Macro {
    pub fn from(loc: Location, name: String, tokens: Vec<Token>) -> Self {
        Self { loc, name, tokens }
    }
}

pub fn compile_tokens_to_operations(tokens: Vec<Token>, include_directories: &Vec<String>) -> Result<Vec<Operation>, io::Error> {
    let mut tokens = tokens.clone();
    tokens.reverse();
    
    let mut operations: Vec<Operation> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut macros: HashMap<String, Macro> = HashMap::new();
    let mut addr_counter: usize = 0;

    while let Some(token) = tokens.pop() {
        // compile token to operation
        match &token.kind {
            TokenKind::Integer(value) => operations.push(Operation::new(OperationKind::PushInt(*value), token.loc)),
            TokenKind::String(value) => operations.push(Operation::new(OperationKind::PushStr(value.clone()), token.loc)),
            TokenKind::Char(value) => operations.push(Operation::new(OperationKind::PushChar(*value), token.loc)),
            TokenKind::Intrinsic(value) => operations.push(Operation::new(OperationKind::Intrinsic(value.clone()), token.loc)),
            TokenKind::Word(value) => {
                if let Some(op_kind) = OperationKind::from_str_builtin(value) {
                    operations.push(Operation::new(op_kind, token.loc.clone()));
                } else if let Some(m) = macros.get_mut(value) {
                    for i in 0..m.tokens.len() {
                        let t = m.tokens.get(i).unwrap();
                        tokens.push(t.clone()); 
                    }
                    continue; // so that the address wont get incremented
                } else {
                    panic!("[ERROR]: Unknown word, identifier {} {:?}", token.loc, token.kind);
                }
            }
            TokenKind::KeyWord(value) => {
                use crate::operation::JUMP_DEFAULT;
                match value {
                    KeywordType::If    => operations.push(Operation::new(OperationKind::If(JUMP_DEFAULT), token.loc)),
                    KeywordType::End   => operations.push(Operation::new(OperationKind::End(JUMP_DEFAULT), token.loc)),
                    KeywordType::Else  => operations.push(Operation::new(OperationKind::Else(JUMP_DEFAULT), token.loc)),
                    KeywordType::While => operations.push(Operation::new(OperationKind::While, token.loc)),
                    KeywordType::Do    => operations.push(Operation::new(OperationKind::Do(JUMP_DEFAULT), token.loc)),
                    KeywordType::Macro => {
                        let tok = tokens.pop().expect("macro must have a body and be closed by `end`");
                        let name = match tok.kind {
                            TokenKind::Word(ref v) => {
                                if let Some(m) = macros.get(v) {
                                    panic!("[ERROR]: {} Macro with identifier {v:?} already implemented at {ml} (can't use the same identifier for macros)", token.loc, ml=&m.loc);
                                }
                                if let Some(op_type) = OperationKind::from_str_builtin(v) {
                                    panic!("[ERROR]: {} Can't use a builtin word {v:?} standing for {op_type:?} as an identifier for `macro`", token.loc);
                                }
                                v.clone()
                            }
                            TokenKind::Integer(v) => panic!("[ERROR]: {} - Expected `macro` identifier `Word` but found `Integer`: {v:?}", token.loc),
                            TokenKind::String(v) => panic!("[ERROR]: {} - Expected `macro` identifier `Word` but found `String`: {v:?}", token.loc),
                            TokenKind::Char(v) => unreachable!("[ERROR]: {} - Expected `macro` identifier `Word` but found `Char`: {v:?}", token.loc),
                            TokenKind::KeyWord(v) => panic!("[ERROR]: {} - Expected `macro` identifier `Word` but found `Keyword`: {v:?}", token.loc),
                            TokenKind::Intrinsic(v) => panic!("[ERROR]: {} - Expected `macro` identifier `Word` but found `Intrinsic`: {v:?}", token.loc),
                        };
                        macros.insert(name.clone(), Macro::from(token.loc, name.clone(), Vec::new()));
                        while let Some(tok) = tokens.pop() {
                            match &tok.kind {
                                TokenKind::KeyWord(value) if *value == KeywordType::End => {
                                    tokens.push(tok);
                                    break;
                                }
                                _ => {}
                            }
                            macros.get_mut(&name).unwrap().tokens.insert(0, tok.clone());
                        }
                        tokens.pop().unwrap_or_else(|| panic!("[ERROR]: {} Macro {name:?} No `end` was found for `macro` block", tok.loc));
                        continue;
                    }
                    KeywordType::Include => {
                        let include_arg = tokens.pop().unwrap();
                        match &include_arg.kind {
                            TokenKind::String(include_file) => {
                                let src_path = PathBuf::from(&token.loc.file);
                                let inc_path = PathBuf::from(&include_file);

                                let src_dir = src_path.parent().unwrap().to_str().unwrap();
                                let inc_dir = inc_path.parent().unwrap().to_str().unwrap();
                                let inc_file_name = inc_path.file_name().unwrap().to_str().unwrap();

                                let mut search_directories = vec![ format!("{}/{}", src_dir, inc_dir) ];
                                let mut include_directories = include_directories.clone();
                                search_directories.append(&mut include_directories);

                                for sd in search_directories.iter() {
                                    if !fs::metadata(&sd).unwrap_or_else(|e| panic!("[ERROR]: {} Directory {inc_dir:?} does not exist: {e}", &include_arg.loc)).is_dir() {
                                        panic!("[ERROR]: {:?} Provided path is not a directory", sd);
                                    }
                                }

                                search_directories.iter_mut().for_each(|d| *d = format!("{}/{}", d, inc_file_name).replace("//", "/") );
                                
                                while let Some(sd) = search_directories.pop() {
                                    match lexer::lex_file_to_tokens(sd.as_str()) {
                                        Ok(ref mut include_tokens) => {
                                            // dbg!(&include_tokens);
                                            include_tokens.reverse();
                                            tokens.append(include_tokens);
                                            break;
                                        }
                                        Err(e) => {
                                            if search_directories.len() <= 1 {
                                                panic!("[ERROR]: {} {:?} {}", include_arg.loc, include_file, e);
                                            }
                                        }
                                    }
                                }
                            }
                            TokenKind::Integer(v) => panic!("[ERROR]: {} {v:?} - File after `include` must be `String` but found `Integer`", token.loc),
                            TokenKind::Word(v) => panic!("[ERROR]: {} {v:?} - File after `include` must be `String` but found `Word`", token.loc),
                            TokenKind::Char(v) => unreachable!("[ERROR]: {} {v:?} - File after `include` must be `String` but found `Char`", token.loc),
                            TokenKind::KeyWord(v) => panic!("[ERROR]: {} {v:?} - File after `include` must be `String` but found `Keyword`", token.loc),
                            TokenKind::Intrinsic(v) => panic!("[ERROR]: {} {v:?} - File after `include` must be `String` but found `Intrinsic`", token.loc),
                        }
                        continue;
                    }
                    // uncaught_keyword_type => panic!("[ERROR]: {} {value:?} Should have already been converted token to operation: {uncaught_keyword_type:?} ", token.loc)
                }
            }
        }

        // crossreference block operations and handle preprocessor
        let mut op = operations.pop().expect("[ERROR]: There are no operations, WTF");
        match &mut op.kind {
            OperationKind::If(_) => stack.push(addr_counter),
            OperationKind::While => stack.push(addr_counter),
            OperationKind::Do(do_jump) => {
                let while_addr = stack.pop().unwrap_or_else(|| panic!("[ERROR]: Found `do` and expected `while`'s address on the stack: {} {}", op.loc, op.address));
                if operations.get(while_addr).unwrap_or_else(|| panic!("[ERROR]: Operation at {while_addr} doesn't exist, expected `while` there")).kind != OperationKind::While {
                    panic!("[ERROR]: {} {:?} found `do` but not `while`, intead found {:?}", &op.loc, &op.kind, operations[while_addr]);
                }
                *do_jump = while_addr as i32;
                stack.push(addr_counter);
            }
            OperationKind::Else(_) => {
                let if_addr = stack.pop().unwrap();
                if let OperationKind::If(ref mut if_jump) = operations.get_mut(if_addr).unwrap_or_else(|| panic!("[ERROR]: Operation at {if_addr} doesn't exist, expected `if` there")).kind {
                    *if_jump = (addr_counter + 1) as i32; 
                }
                stack.push(addr_counter);
            }
            OperationKind::End(end_jump) => {
                let prev_addr = stack.pop().unwrap_or_else(|| panic!("[ERROR]: Found `end` but no corresponding block keyword address on the stack"));
                match &mut operations.get_mut(prev_addr).unwrap_or_else(|| panic!("[ERROR]: Operation at {prev_addr} doesn't exist, expected block forming keyword")).kind {
                    OperationKind::If(if_jump) => *if_jump = addr_counter as i32,
                    OperationKind::Else(else_jump) => *else_jump = addr_counter as i32,
                    OperationKind::Do(do_jump) => {
                        *end_jump = *do_jump;
                        *do_jump = (addr_counter + 1) as i32;
                    }
                    _ => panic!("[ERROR] `end` keyword found with no preceding block operations")
                }
            }
            _ => {}
        }
        
        op.address = addr_counter;
        operations.push(op);
        addr_counter += 1;
    }
    
    Ok(operations)
}

