use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use crate::operation::{OperationType, Operation};
use crate::token::{Token, TokenType, TokenValue};

pub fn lex_file(file: &str) -> Vec<Token> {
    // println!("[INFO]: reading the program from '{}'", file);
    let lines = read_lines(file);
    return lines
        .enumerate()
        .flat_map(|(row, line)| lex_line(file, row, line.unwrap()))
        .collect();
}

fn lex_line(file: &str, row: usize, line: String) -> Vec<Token>
{
    // println!("[INFO]: Original ({row}): {:?}", line);
    let line = line
        .split_at(line.find("//")
        .unwrap_or_else(|| line.len()))
        .0.to_string();
    // println!("[INFO]: UnComment ({row}): {:?}", line);

    let mut col: usize = find_col(&line, 0, |x| x != b' ');
    let mut col_end: usize;
    let mut tokens: Vec<Token> = Vec::new();
    
    while col < line.len() {
        let tok: _;
        if line.as_bytes()[col] == b'"' { // Parsing String literal
            col_end = find_col(&line, col + 1, |x| x == b'"');
            assert!(line.as_bytes()[col] == b'"');
            assert!(line.as_bytes()[col_end] == b'"');

            let value = &line[(col + 1)..=(col_end - 1)].to_string();
            tok = Token::new_str(value, &file.to_string(), row, col);

            col = find_col(&line, col_end + 1, |x| x != b' ');
        } else {
            col_end = find_col(&line, col, |x| x == b' ');

            let word = &line[col..col_end]; 
            tok = match word.parse::<i32>() {
                Ok(v)  => Token::new_int(v, &file.to_string(), row, col),
                Err(_) => Token::new_word(&word.to_string(), &file.to_string(), row, col),
            };
            
            col = find_col(&line, col_end, |x| x != b' ');
        }
        tokens.push(tok);
    }

    return tokens;
}

fn crossreference_blocks(program: &mut Vec<Operation>) {
    let mut stack: Vec<usize> = Vec::new();
    
    // process block instructions
    for ip in 0..program.len() {
        let op_type = program[ip].op_type;
        match op_type  { 
            OperationType::If => stack.push(ip),
            OperationType::While => stack.push(ip),
            OperationType::Do => {
                let while_ip = stack.pop().unwrap();
                program[ip].jump = while_ip as i32;
                stack.push(ip);
            }
            OperationType::Else => {
                let if_index = stack.pop().unwrap();
                program[if_index].jump = (ip + 1) as i32; 
                stack.push(ip);
            }
            OperationType::End => {
                let prev_index = stack.pop().unwrap();
                if program[prev_index].op_type == OperationType::If || program[prev_index].op_type == OperationType::Else {
                    program[prev_index].jump = ip as i32;
                }
                if program[prev_index].op_type == OperationType::Do {
                    program[ip].jump = program[prev_index].jump as i32;
                    program[prev_index].jump = ( ip + 1) as i32;
                }
            }
            _ => {} // ignore other instructions
        }
    }
}

pub fn parse_tokens_to_operations(tokens: &Vec<Token>) -> Vec<Operation> {
    let mut operations = tokens.iter()
        .enumerate()
        .map(|(index, token)| parse_token_to_operation(index, token))
        .collect();
    crossreference_blocks(&mut operations);
    operations
}

fn parse_token_to_operation(index: usize, token: &Token) -> Operation {
    return match (&token.tok_type, &token.value) {
        (TokenType::Int, TokenValue::Int(value)) => {
            Operation::new_value_int(index, OperationType::PushInt, *value, &token.loc)
        }
        (TokenType::Str, TokenValue::Str(value)) => {
            Operation::new_value_str(index, OperationType::PushStr, value, &token.loc)
        }
        (TokenType::Word, TokenValue::Str(value)) => {
            match value.as_str() {
                "dump" => Operation::new_value_none(index, OperationType::Dump, &token.loc),
                "dup"  => Operation::new_value_none(index, OperationType::Duplicate, &token.loc),
                "dup2" => Operation::new_value_none(index, OperationType::Duplicate2, &token.loc),
                "drop" => Operation::new_value_none(index, OperationType::Drop, &token.loc),
                "swap" => Operation::new_value_none(index, OperationType::Swap, &token.loc),
                "over" => Operation::new_value_none(index, OperationType::Over, &token.loc),
                
                "+" | "add" => Operation::new_value_none(index, OperationType::Add, &token.loc),
                "-" | "sub" => Operation::new_value_none(index, OperationType::Subtract, &token.loc),
                "*" | "mul" => Operation::new_value_none(index, OperationType::Multiply, &token.loc),
                "/" | "div" => Operation::new_value_none(index, OperationType::Divide, &token.loc),
                "%" | "mod" => Operation::new_value_none(index, OperationType::Modulo, &token.loc),
                
                "="  | "eq"  => Operation::new_value_none(index, OperationType::Equal, &token.loc),
                "!=" | "neq" => Operation::new_value_none(index, OperationType::NotEqual, &token.loc),
                "<"  | "le"  => Operation::new_value_none(index, OperationType::Less, &token.loc),
                ">"  | "gr"  => Operation::new_value_none(index, OperationType::Greater, &token.loc),
                "<=" | "eql" => Operation::new_value_none(index, OperationType::LessEqual, &token.loc),
                ">=" | "egr" => Operation::new_value_none(index, OperationType::GreaterEqual, &token.loc),
                "!"  | "not" => Operation::new_value_none(index, OperationType::Not, &token.loc),
                
                "<<" | "shl"  => Operation::new_value_none(index, OperationType::ShiftLeft, &token.loc),
                ">>" | "shr"  => Operation::new_value_none(index, OperationType::ShiftRight, &token.loc),
                "&"  | "band" => Operation::new_value_none(index, OperationType::BitAnd, &token.loc),
                "|"  | "bor"  => Operation::new_value_none(index, OperationType::BitOr, &token.loc),
                
                "end"   => Operation::new_value_none(index, OperationType::End, &token.loc),
                "if"    => Operation::new_value_none(index, OperationType::If, &token.loc),
                "else"  => Operation::new_value_none(index, OperationType::Else, &token.loc),
                "while" => Operation::new_value_none(index, OperationType::While, &token.loc),
                "do"    => Operation::new_value_none(index, OperationType::Do, &token.loc),
                
                "mem"         => Operation::new_value_none(index, OperationType::MemoryPush, &token.loc),
                "," | "load"  => Operation::new_value_none(index, OperationType::MemoryLoad, &token.loc),
                "." | "store" => Operation::new_value_none(index, OperationType::MemoryStore, &token.loc),
                
                "syscall1" => Operation::new_value_none(index, OperationType::Syscall1, &token.loc),
                "syscall3" => Operation::new_value_none(index, OperationType::Syscall3, &token.loc),
                
                _ => panic!("Unknown word with value {:?}", value)
            }
        }
        (tok_type, tok_value) => panic!("Unknown TokenType and TokenValue combination: type: {:?} value: {:?}", tok_type, tok_value)
    }
}

fn read_lines<P>(filename: P) 
    -> io::Lines<io::BufReader<File>>
where P: AsRef<std::path::Path> 
{
    let file = File::open(filename).unwrap_or_else(|e| panic!("[ERROR]: {e}") );
    io::BufReader::new(file).lines()
}

fn find_col(line: &String, index: usize, find: impl Fn(u8) -> bool) -> usize {
    let mut i = index;
    while i < line.len() && !find(line.as_bytes()[i]) {
        if i >= line.len() {
            panic!("Reached the end of the line without satisfying the predicate");
        }
        i += 1;
    }
    i
}
