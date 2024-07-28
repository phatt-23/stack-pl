use std::{self, io, fs, path::PathBuf};
use std::collections::HashMap;

use crate::operation::{Operation, OperationKind};
use crate::token::{Token, TokenKind};
use crate::keyword::KeywordType;
use crate::location::Location;
use crate::lexer;

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    pub loc: Location,
    pub name: String,
    pub tokens: Vec<Token>,
}

impl Macro {
    pub const DEFAULT_EXPANSION_LIMIT: usize = 1000;
    pub fn from(loc: Location, name: String, tokens: Vec<Token>) -> Self {
        Self { loc, name, tokens }
    }
}

pub fn compile_tokens_to_operations(
    tokens: Vec<Token>, 
    include_directories: &Vec<String>, 
    expansion_limit: usize
) -> Result<Vec<Operation>, io::Error> {
    let mut tokens = tokens.clone();
    tokens.reverse();
    
    let mut operations:     Vec<Operation>          = Vec::new();
    let mut stack:          Vec<usize>              = Vec::new();
    let mut macros:         HashMap<String, Macro>  = HashMap::new();
    let mut includes:       HashMap<String, usize>  = HashMap::new();
    let mut addr_counter:   usize                   = 0;

    while let Some(token) = tokens.pop() {
        // compile token to operation
        match &token.kind {
            TokenKind::Integer32(value) => operations.push(Operation::new(OperationKind::PushInt32(*value), token.loc)),
            TokenKind::Integer64(value) => operations.push(Operation::new(OperationKind::PushInt64(*value), token.loc)),
            TokenKind::String(value)    => operations.push(Operation::new(OperationKind::PushStr(value.clone()), token.loc)),
            TokenKind::Char(value)      => operations.push(Operation::new(OperationKind::PushChar(*value), token.loc)),
            TokenKind::Intrinsic(value) => operations.push(Operation::new(OperationKind::Intrinsic(value.clone()), token.loc)),
            TokenKind::Word(value)      => {
                if let Some(op_kind) = OperationKind::from_str_builtin(value) {
                    operations.push(Operation::new(op_kind, token.loc.clone()));
                } else if let Some(m) = macros.get_mut(value) {
                    for t in m.tokens.iter_mut() {
                        t.expanded += 1;
                        assert!(t.expanded < expansion_limit, 
                            "[ERROR]: {} Macro {:?} from {} was expanded beyond the macro expansion limit of {}!", 
                            token.loc, value, t.loc, expansion_limit
                        );
                        tokens.push(t.clone()); 
                    }
                    continue;//So that the addr_counter wont get incremented
                } else {
                    panic!("[ERROR]: {} Unknown word (identifier) {:?}!", token.loc, token.kind);
                }
            }
            TokenKind::KeyWord(value) => {
                match value {
                    KeywordType::If    => operations.push(Operation::new(OperationKind::If(None), token.loc)),
                    KeywordType::End   => operations.push(Operation::new(OperationKind::End(None), token.loc)),
                    KeywordType::Else  => operations.push(Operation::new(OperationKind::Else(None), token.loc)),
                    KeywordType::While => operations.push(Operation::new(OperationKind::While, token.loc)),
                    KeywordType::Do    => operations.push(Operation::new(OperationKind::Do(None), token.loc)),
                    KeywordType::Macro => {
                        assert!(!tokens.is_empty(), "[ERROR]: {} Found keyword `macro` followed by nothing!", token.loc);
                        let tok = tokens.pop().unwrap();
                        let name = match tok.kind {
                            TokenKind::Word(ref v) => {
                                if let Some(m) = macros.get(v) {
                                    panic!("[ERROR]: {} Macro with identifier {:?} already implemented at {} (can't use the same identifier for macros)!", token.loc, v, m.loc);
                                }
                                if let Some(op_type) = OperationKind::from_str_builtin(v) {
                                    panic!("[ERROR]: {} Macro identifier can't be a builtin word (intrinsic) {v:?} (stands for {:?})!", token.loc, op_type);
                                }
                                v.clone()
                            }
                            TokenKind::Integer32(v) => panic!("[ERROR]: {} ({v:?}) Identifier after macro must be `Word` but found `Integer32`!", token.loc),
                            TokenKind::Integer64(v) => panic!("[ERROR]: {} ({v:?}) Identifier after macro must be `Word` but found `Integer64`!", token.loc),
                            TokenKind::String(v)    => panic!("[ERROR]: {} ({v:?}) Identifier after macro must be `Word` but found `String`!", token.loc),
                            TokenKind::Char(v)      => panic!("[ERROR]: {} ({v:?}) Identifier after macro must be `Word` but found `Char`!", token.loc),
                            TokenKind::KeyWord(v)   => panic!("[ERROR]: {} ({v:?}) Identifier after macro must be `Word` but found `Keyword`!", token.loc),
                            TokenKind::Intrinsic(v) => panic!("[ERROR]: {} ({v:?}) Identifier after macro must be `Word` but found `Intrinsic`!", token.loc),
                        };
                        macros.insert(name.clone(), Macro::from(token.loc, name.clone(), Vec::new()));
                        let mut expanded_tokens: Vec<Token> = vec![];
                        let mut nesting_depth = 0;
                        while let Some(tok) = tokens.pop() {
                            match &tok.kind {
                                TokenKind::KeyWord(value) => {
                                    if *value == KeywordType::End && nesting_depth == 0 {
                                        tokens.push(tok);
                                        break;
                                    } else {
                                        if [KeywordType::If, KeywordType::While, KeywordType::Macro].contains(value) {
                                            nesting_depth += 1;
                                        } else if *value == KeywordType::End {
                                            nesting_depth -= 1;
                                        }
                                        expanded_tokens.push(tok);
                                    }
                                }
                                _ => expanded_tokens.push(tok),
                            }
                        }
                        expanded_tokens.reverse();
                        macros.get_mut(&name).unwrap().tokens = expanded_tokens;
                        tokens.pop().unwrap_or_else(|| panic!("[ERROR]: {} Macro {:?} has no `end`!", tok.loc, name));
                        continue;
                    }
                    KeywordType::Include => {
                        let include_arg = tokens.pop().unwrap();
                        match &include_arg.kind {
                            TokenKind::String(include_file) => {
                                // todo: decide if this is possible, if yes then code like this 'inlcude "  stdlib.p "' will pass, if not it wont
                                let include_file = include_file.trim();
                                assert!(include_file.ends_with(crate::utils::LANGUAGE_SUFFIX), "[ERROR]: {} Source files have to end with {:?} suffix.", include_arg.loc, crate::utils::LANGUAGE_SUFFIX);
                                let src_path = PathBuf::from(&token.loc.file);
                                let inc_path = PathBuf::from(&include_file);

                                let src_dir = src_path.parent().unwrap().to_str().unwrap();
                                let inc_dir = inc_path.parent().unwrap().to_str().unwrap();
                                let inc_file_name = inc_path.file_name().unwrap().to_str().unwrap();

                                let mut search_directories = vec![ format!("{}/{}", src_dir, inc_dir) ];
                                let mut include_directories = include_directories.to_owned();
                                search_directories.append(&mut include_directories);

                                for sd in search_directories.iter() {
                                    assert!(
                                        fs::metadata(sd).unwrap_or_else(|e| panic!("[ERROR]: {} Directory {:?} does not exist! {e}", &include_arg.loc, inc_dir)).is_dir(), 
                                        "[ERROR]: {:?} Provided path is not a directory!", sd
                                    );
                                }

                                let mut search_filepaths: Vec<String> = search_directories.iter().map(|d| format!("{}/{}", d, inc_file_name).replace("//", "/") ).collect();

                                let mut include_success = false;
                                while let Some(fp) = search_filepaths.pop() {
                                    if let Ok(ref mut include_tokens) = lexer::lex_file_to_tokens(fp.as_str()) {
                                        include_tokens.reverse();
                                        tokens.append(include_tokens);
                                        if includes.contains_key(&fp.to_string()) {
                                            assert!(
                                                *includes.get(&fp.to_string()).unwrap() < expansion_limit,
                                                "[ERROR]: {} For file {:?} include limit (expansion limit of {}) has been exceeded!", include_arg.loc, include_file, expansion_limit
                                            );
                                            includes.insert(fp.to_string(), *includes.get(&fp.to_string()).unwrap() + 1);
                                        } else {
                                            includes.insert(fp.to_string(), 0);
                                        }
                                        include_success = true;
                                        break;
                                    }
                                }
                                assert!(include_success, "[ERROR]: {} Include for {:?} failed, no such file in path!", token.loc, include_file);
                            }
                            TokenKind::Integer32(v) => panic!("[ERROR]: {} {v:?} File after `include` must be `String` but found `Integer32`", token.loc),
                            TokenKind::Integer64(v) => panic!("[ERROR]: {} {v:?} File after `include` must be `String` but found `Integer64`", token.loc),
                            TokenKind::Word(v)      => panic!("[ERROR]: {} {v:?} File after `include` must be `String` but found `Word`", token.loc),
                            TokenKind::Char(v)      => panic!("[ERROR]: {} {v:?} File after `include` must be `String` but found `Char`", token.loc),
                            TokenKind::KeyWord(v)   => panic!("[ERROR]: {} {v:?} File after `include` must be `String` but found `Keyword`", token.loc),
                            TokenKind::Intrinsic(v) => panic!("[ERROR]: {} {v:?} File after `include` must be `String` but found `Intrinsic`", token.loc),
                        }
                        continue;
                    }
                }
            }
        }

        // crossreference block operations and handle preprocessor
        let mut op = operations.pop().expect("[ERROR]: There are no operations, WTF");
        match &mut op.kind {
            OperationKind::If(_) => stack.push(addr_counter),
            OperationKind::While => stack.push(addr_counter),
            OperationKind::Do(do_jump) => {
                let while_addr = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} Found `do` with no preceding `while` (address stack is empty)! {}", op.loc, op.address));
                
                let while_op = operations.get(while_addr).unwrap_or_else(|| panic!("[ERROR]: {} There is no operation at {while_addr}, expected `while` operation there!", op.loc));
                
                assert!(
                    while_op.kind == OperationKind::While, 
                    "[ERROR]: {} Found `do` without preceeding `while`, intead found {} {:?}!", op.loc, while_op.loc, while_op.kind
                );

                *do_jump = Some(while_addr);
                stack.push(addr_counter);
            }
            OperationKind::Else(_) => {
                let if_addr = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} Found `else` with no preceding `if` (address stack is empty)! {}", op.loc, op.address));
                
                let if_op = operations.get_mut(if_addr).unwrap_or_else(|| panic!("[ERROR]: {} There is no operation at {if_addr}, expected `if` operation there!", op.loc));

                if let OperationKind::If(ref mut if_jump) = if_op.kind {
                    *if_jump = Some(addr_counter + 1); 
                } else {
                    panic!("[ERROR]: {} Found `do` without preceeding `if`, instead found {} {:?}!", op.loc, if_op.loc, if_op.kind);
                }
                
                stack.push(addr_counter);
            }
            OperationKind::End(end_jump) => {
                let prev_addr = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} Found `end` without preceeding block forming keyword (address stack is empty)!", op.loc));

                let prev_op = operations.get_mut(prev_addr).unwrap_or_else(|| panic!("[ERROR]: {} There is no operation at {prev_addr}, expected block forming keyword there!", op.loc));

                match &mut prev_op.kind {
                    OperationKind::If   (if_jump)   => *if_jump   = Some(addr_counter),
                    OperationKind::Else (else_jump) => *else_jump = Some(addr_counter),
                    OperationKind::Do   (do_jump)   => {
                        *end_jump = *do_jump;
                        *do_jump  = Some(addr_counter + 1);
                    }
                    _ => unreachable!("[ERROR]: Unreachable: {} Keyword `end` found with no preceding block opeations!", op.loc)
                }
            }
            _ => {}//Do nothing for non-block forming operations
        }
        op.address = addr_counter;
        operations.push(op);
        addr_counter += 1;
    }

    for op in &operations {
        //Check if all block forming operations have valid jump addresses
        match op.kind {
            OperationKind::If(jump) if jump.is_none() => {
                unreachable!("[ERROR]: {} Operation `if` has to have a set jump address!", op.loc);
            }
            OperationKind::Else(jump) if jump.is_none() => {
                unreachable!("[ERROR]: {} Operation `else` operation has to have a set jump address!", op.loc);
            }
            OperationKind::Do(jump) if jump.is_none() => {
                unreachable!("[ERROR]: {} Operation do` operation has to have a set jump address!", op.loc);
            }
            _ => {}//Others dont have to have their jump address set
        }
    }
    
    Ok(operations)
}

