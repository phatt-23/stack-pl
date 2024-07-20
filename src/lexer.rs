use std::fs::File;
use std::io::{self, BufRead};
use super::operation::{OperationType, Operation};
use super::token::{Token, TokenType, TokenValue};

pub fn lex_file(file: &str) -> Vec<Token> {
    let mut program: Vec<Token> = Vec::new();
    if let Ok(lines) = read_lines(file) {
        println!("[INFO]: reading the program from '{}'", file);
        for (row, line) in lines.enumerate() {
            match line {
                Ok(line) => program = [program, lex_line(file, row, line)].concat(),
                Err(err) => panic!("[ERROR]: {}", err),
            }
        }
    }
    program
}

fn lex_line(file: &str, row: usize, line: String) -> Vec<Token>
{
    let mut program: Vec<Token> = Vec::new();
    let inter: Vec<&str> = line.split("//").take(1).collect();
    let words: Vec<&str> = inter.iter().flat_map(|&s| s.split_whitespace()).collect();
    println!("[INFO]: Raw line ({row}): {:?}", line);
    println!("[INFO]: Splitted ({row}): {:?}", words);

    let mut col: usize = 0;
    for word in words {
        col = step_col(&line, col, |x| x != b' ');
        let token = lex_word(word, file, row, col);
        program.push(token);
        col = step_col(&line, col, |x| x == b' ');
    }
    program
}

fn lex_word(word: &str, file: &str, row: usize, col: usize) -> Token {
    let value = word.parse::<i64>(); // try tokenizing to an Integer type
    match value {
        Ok(v) => {
            Token::new_int(TokenType::Int, v, &file.to_string(), row, col)         
        }
        Err(_) => {
            Token::new_word(TokenType::Word, &word.to_string(), &file.to_string(), row, col)
        }
    }
}

fn crossreference_blocks(program: &mut Vec<Operation>) {
    let mut stack: Vec<usize> = Vec::new();
        
    for ip in 0..program.len() {
        match program[ip].op_type {
            // process block instructions
            OperationType::If => stack.push(ip),
            OperationType::While => stack.push(ip),
            OperationType::Do => {
                let while_index = stack.pop().unwrap();
                let while_op = &program[while_index];
                program[ip].value = while_op.index as i64;
                stack.push(ip);
            }
            OperationType::Else => {
                let if_index = stack.pop().unwrap();
                program[if_index].value = (ip + 1) as i64; 
                stack.push(ip);
            }
            OperationType::End => {
                let prev_index = stack.pop().unwrap();
                if program[prev_index].op_type == OperationType::If || program[prev_index].op_type == OperationType::Else {
                    program[prev_index].value =  program[ip].index as i64;
                }
                if program[prev_index].op_type == OperationType::Do {
                    program[ip].value = program[prev_index].value as i64;
                    program[prev_index].value = ( program[ip].index + 1) as i64;
                }
            }
            // ignore other instructions
            OperationType::Push 
                | OperationType::Dump | OperationType::Drop | OperationType::Swap | OperationType::Over
                | OperationType::Duplicate | OperationType::Duplicate2
                | OperationType::Add | OperationType::Subtract 
                | OperationType::Multiply | OperationType::Divide
                | OperationType::Modulo
                | OperationType::Equal | OperationType::NotEqual | OperationType::Not 
                | OperationType::Less | OperationType::Greater 
                | OperationType::GreaterEqual | OperationType::LessEqual
                | OperationType::BitAnd | OperationType::BitOr 
                | OperationType::ShiftRight | OperationType::ShiftLeft 
                | OperationType::MemoryPush | OperationType::MemoryLoad | OperationType::MemoryStore
                | OperationType::Syscall1 | OperationType::Syscall3
                => {} 
        }
    }
}

pub fn parse_tokens_to_operations(tokens: &Vec<Token>) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();
    let mut index: usize = 0;
    for token in tokens {
        let op = parse_token_to_operation(token, index);
        operations.push(op);
        index += 1;
    }
    crossreference_blocks(&mut operations);
    operations
}

fn parse_token_to_operation(token: &Token, index: usize) -> Operation {
    return match token.tok_type {
        TokenType::Word => {
            match &token.value {
                TokenValue::Str(s) => {
                    match s.as_str() {
                        "dump" => Operation::new(OperationType::Dump, index, &token.loc),
                        "dup"  => Operation::new(OperationType::Duplicate, index, &token.loc),
                        "dup2" => Operation::new(OperationType::Duplicate2, index, &token.loc),
                        "drop" => Operation::new(OperationType::Drop, index, &token.loc),
                        "swap" => Operation::new(OperationType::Swap, index, &token.loc),
                        "over" => Operation::new(OperationType::Over, index, &token.loc),
                        
                        "+" | "add" => Operation::new(OperationType::Add, index, &token.loc),
                        "-" | "sub" => Operation::new(OperationType::Subtract, index, &token.loc),
                        "*" | "mul" => Operation::new(OperationType::Multiply, index, &token.loc),
                        "/" | "div" => Operation::new(OperationType::Divide, index, &token.loc),
                        "%" | "mod" => Operation::new(OperationType::Modulo, index, &token.loc),
                        
                        "="  | "eq"  => Operation::new(OperationType::Equal, index, &token.loc),
                        "!=" | "neq" => Operation::new(OperationType::NotEqual, index, &token.loc),
                        "<"  | "le"  => Operation::new(OperationType::Less, index, &token.loc),
                        ">"  | "gr"  => Operation::new(OperationType::Greater, index, &token.loc),
                        "<=" | "eql" => Operation::new(OperationType::LessEqual, index, &token.loc),
                        ">=" | "egr" => Operation::new(OperationType::GreaterEqual, index, &token.loc),
                        "!"  | "not" => Operation::new(OperationType::Not, index, &token.loc),
                        
                        "<<" | "shl"  => Operation::new(OperationType::ShiftLeft, index, &token.loc),
                        ">>" | "shr"  => Operation::new(OperationType::ShiftRight, index, &token.loc),
                        "&"  | "band" => Operation::new(OperationType::BitAnd, index, &token.loc),
                        "|"  | "bor"  => Operation::new(OperationType::BitOr, index, &token.loc),
                        
                        "end"   => Operation::new(OperationType::End, index, &token.loc),
                        "if"    => Operation::new(OperationType::If, index, &token.loc),
                        "else"  => Operation::new(OperationType::Else, index, &token.loc),
                        "while" => Operation::new(OperationType::While, index, &token.loc),
                        "do"    => Operation::new(OperationType::Do, index, &token.loc),
                        
                        "mem"           => Operation::new(OperationType::MemoryPush, index, &token.loc),
                        "," | "load"  => Operation::new(OperationType::MemoryLoad, index, &token.loc),
                        "." | "store" => Operation::new(OperationType::MemoryStore, index, &token.loc),
                        
                        "syscall1" => Operation::new(OperationType::Syscall1, index, &token.loc),
                        "syscall3" => Operation::new(OperationType::Syscall3, index, &token.loc),
                        _ => panic!("[ERROR]: {} {:?} is unknown operation", token.loc, s)
                    }
                }
                _ => panic!("[ERROR]: TokenType::Word must have TokenValue::Str")
            }
        }

        TokenType::Int => {
            match token.value {
                TokenValue::Int(num) => Operation::new_with_value(OperationType::Push, num, index, &token.loc),
                _ => panic!("[ERROR]: TokenType::Int must have TokenValue::Str")
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<std::path::Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn step_col(line: &String, index: usize, until: impl Fn(u8) -> bool) -> usize {
    let mut i = index;
    while i < line.len() && until(line.as_bytes()[i]) {
        i += 1;
    }
    i
}
