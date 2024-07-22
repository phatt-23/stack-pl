use std::collections::HashMap;

use crate::operation::{Operation, OperationType};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
struct Macro {
    pub loc: crate::location::Location,
    pub name: String,
    pub tokens: Vec<Token>,
}

pub fn compile_tokens_to_operations(tokens: &Vec<Token>) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut macros: HashMap<String, Macro> = HashMap::new();

    let mut ip: usize = 0;
    let mut tokens = tokens.clone();
    tokens.reverse();
    while tokens.len() > 0 {
        let token = tokens.pop().unwrap();
        // compile token to operation
        let op = match &token.kind {
            TokenKind::Integer(value) => Operation::new_value_int(ip, OperationType::PushInt, *value, &token.loc),
            TokenKind::String(value) => Operation::new_value_str(ip, OperationType::PushStr, value, &token.loc),
            TokenKind::Word(value) => {
                if let Some(op_type) = OperationType::from_str(value) {
                    Operation::new_value_none(ip, op_type, &token.loc)
                } else {
                    if let Some(m) = macros.get_mut(value) {
                        // println!("handle macro expansion {m:?}");
                        for i in 0..m.tokens.len() {
                            let t = m.tokens.get(i).unwrap();
                            tokens.push(t.clone()); 
                            // println!("[INFO]: inserted {t} to tokens");
                        }
                        
                        continue;   
                    }
                    // println!("{value:?} handle unknown words, are they in macros vector? if yes then expand it, if not they truly are forbidden");
                    // nothing matches
                    panic!("[ERROR]: Unknown word, identifier {} {:?}", token.loc, token.kind);
                }
            }
        };
        operations.push(op.clone());
        match op.op_type {
            OperationType::If => stack.push(ip),
            OperationType::While => stack.push(ip),
            OperationType::Do => {
                let while_ip = stack.pop().unwrap();
                assert!(operations[while_ip].op_type == OperationType::While);
                operations[ip].jump = while_ip as i32;
                stack.push(ip);
            }
            OperationType::Else => {
                let if_ip = stack.pop().unwrap();
                assert!(operations[if_ip].op_type == OperationType::If);
                operations[if_ip].jump = (ip + 1) as i32; 
                stack.push(ip);
            }
            OperationType::Macro => {
                // find out if macro name is free, add it to macros vector, append tokens to Macro struct till `end` is found
                assert!(tokens.len() > 0, "macro must have a body and `end`");
                // get name
                let name = match tokens.pop().unwrap().kind {
                    TokenKind::Word(v) => {
                        match macros.get(&v) {
                            Some(m) => panic!(
                                "[ERROR]: {ol} Macro with identifier {v:?} already implemented at {ml} (can't use the same identifier for macros)", ol=&op.loc, ml=&m.loc
                            ),
                            None => {
                                // check if macro name is one of the keywords
                                if let Some(op_type) = OperationType::from_str(&v) {
                                    panic!("[ERROR]: {} Can't use a builtin keyword {v:?} of {op_type:?} as an identifier for `macro`", &op.loc);
                                }
                                v
                            }
                        }
                    }
                    TokenKind::Integer(v) => panic!("[ERROR]: {} {v:?} - Expected `macro` identifier `Word` but found `Integer`", &op.loc),
                    TokenKind::String(v) => panic!("[ERROR]: {} {v:?} - Expected `macro` identifier `Word` but found `String`", &op.loc),
                };
                
                macros.insert(
                    name.clone(), 
                    Macro { 
                        loc: op.loc.clone(), 
                        name: name.clone(), 
                        tokens: Vec::new() 
                });

                while let Some(tok) = tokens.pop() {
                    if matches!(&tok.kind, TokenKind::Word(v) if v == "end") {
                        tokens.push(tok);
                        break;
                    }
                    macros.get_mut(&name).unwrap().tokens.insert(0, tok.clone());
                }
                
                let end = tokens.pop()
                    .unwrap_or_else(|| panic!("[ERROR]: {} Macro {name:?} No `end` was found for `macro` block", &op.loc));

                assert!(matches!(&end.kind, TokenKind::Word(v) if v == "end"));
                
                // println!("[DBG]: removing the macro from operaions");
                operations.pop();
                // todo!("macro added succesfully {m:#?}");
            }
            OperationType::End => {
                let prev_ip = stack.pop().unwrap();
                if operations[prev_ip].op_type == OperationType::If || operations[prev_ip].op_type == OperationType::Else {
                    operations[prev_ip].jump = ip as i32;
                }
                if operations[prev_ip].op_type == OperationType::Do {
                    operations[ip].jump = operations[prev_ip].jump as i32;
                    operations[prev_ip].jump = ( ip + 1) as i32;
                }
            }
            _ => {} // ignore other instructions
        }
        
        ip += 1;
    }

    // for ip in 0..operations.len() {
    //     let op = operations.get(ip).unwrap();
    // }
    return operations;
}

// pub fn crossreference_blocks(program: &mut Vec<Operation>) {
//     let mut stack: Vec<usize> = Vec::new();
    
//     // process block instructions
//     for ip in 0..program.len() {
//         let op_type = program[ip].op_type;
//         match op_type  {
//             OperationType::If => stack.push(ip),
//             OperationType::While => stack.push(ip),
//             OperationType::Do => {
//                 let while_ip = stack.pop().unwrap();
//                 assert!(program[while_ip].op_type == OperationType::While);
//                 program[ip].jump = while_ip as i32;
//                 stack.push(ip);
//             }
//             OperationType::Else => {
//                 let if_ip = stack.pop().unwrap();
//                 assert!(program[if_ip].op_type == OperationType::If);
//                 program[if_ip].jump = (ip + 1) as i32; 
//                 stack.push(ip);
//             }
//             OperationType::Macro => {
//                 todo!();
//             }
//             OperationType::End => {
//                 let prev_ip = stack.pop().unwrap();
//                 if program[prev_ip].op_type == OperationType::If || program[prev_ip].op_type == OperationType::Else {
//                     program[prev_ip].jump = ip as i32;
//                 }
//                 if program[prev_ip].op_type == OperationType::Do {
//                     program[ip].jump = program[prev_ip].jump as i32;
//                     program[prev_ip].jump = ( ip + 1) as i32;
//                 }
//             }
//             _ => {} // ignore other instructions
//         }
//     }
// }