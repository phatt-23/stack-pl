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

    let mut col = step_col_while(&line, 0, |x| x == b' ');
    for word in words {
        col = step_col_while(&line, col, |x| x != b' ');
        let token = lex_word(word, file, row, col);
        program.push(token);
        col = step_col_while(&line, col, |x| x == b' ');
    }
    program
}

fn lex_word(word: &str, file: &str, row: usize, col: usize) -> Token {
    let value = word.parse::<i32>(); // try tokenizing to an Integer type
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
    let mut operations: Vec<Operation> = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let op = parse_token_to_operation(index, token);
        operations.push(op);
    }
    crossreference_blocks(&mut operations);
    operations
}

fn parse_token_to_operation(index: usize, token: &Token) -> Operation {
    return match token.tok_type {
        TokenType::Word => {
            match &token.value {
                TokenValue::Str(s) => {
                    match s.as_str() {
                        "dump" => Operation::new(index, OperationType::Dump, &token.loc),
                        "dup"  => Operation::new(index, OperationType::Duplicate, &token.loc),
                        "dup2" => Operation::new(index, OperationType::Duplicate2, &token.loc),
                        "drop" => Operation::new(index, OperationType::Drop, &token.loc),
                        "swap" => Operation::new(index, OperationType::Swap, &token.loc),
                        "over" => Operation::new(index, OperationType::Over, &token.loc),
                        
                        "+" | "add" => Operation::new(index, OperationType::Add, &token.loc),
                        "-" | "sub" => Operation::new(index, OperationType::Subtract, &token.loc),
                        "*" | "mul" => Operation::new(index, OperationType::Multiply, &token.loc),
                        "/" | "div" => Operation::new(index, OperationType::Divide, &token.loc),
                        "%" | "mod" => Operation::new(index, OperationType::Modulo, &token.loc),
                        
                        "="  | "eq"  => Operation::new(index, OperationType::Equal, &token.loc),
                        "!=" | "neq" => Operation::new(index, OperationType::NotEqual, &token.loc),
                        "<"  | "le"  => Operation::new(index, OperationType::Less, &token.loc),
                        ">"  | "gr"  => Operation::new(index, OperationType::Greater, &token.loc),
                        "<=" | "eql" => Operation::new(index, OperationType::LessEqual, &token.loc),
                        ">=" | "egr" => Operation::new(index, OperationType::GreaterEqual, &token.loc),
                        "!"  | "not" => Operation::new(index, OperationType::Not, &token.loc),
                        
                        "<<" | "shl"  => Operation::new(index, OperationType::ShiftLeft, &token.loc),
                        ">>" | "shr"  => Operation::new(index, OperationType::ShiftRight, &token.loc),
                        "&"  | "band" => Operation::new(index, OperationType::BitAnd, &token.loc),
                        "|"  | "bor"  => Operation::new(index, OperationType::BitOr, &token.loc),
                        
                        "end"   => Operation::new(index, OperationType::End, &token.loc),
                        "if"    => Operation::new(index, OperationType::If, &token.loc),
                        "else"  => Operation::new(index, OperationType::Else, &token.loc),
                        "while" => Operation::new(index, OperationType::While, &token.loc),
                        "do"    => Operation::new(index, OperationType::Do, &token.loc),
                        
                        "mem"         => Operation::new(index, OperationType::MemoryPush, &token.loc),
                        "," | "load"  => Operation::new(index, OperationType::MemoryLoad, &token.loc),
                        "." | "store" => Operation::new(index, OperationType::MemoryStore, &token.loc),
                        
                        "syscall1" => Operation::new(index, OperationType::Syscall1, &token.loc),
                        "syscall3" => Operation::new(index, OperationType::Syscall3, &token.loc),
                        _ => panic!("[ERROR]: {} {:?} is unknown word", token.loc, s)
                    }
                }
                _ => panic!("[ERROR]: TokenType::Word must have TokenValue::Str")
            }
        }

        TokenType::Int => {
            match token.value {
                TokenValue::Int(num) => Operation::new_with_value(index, OperationType::Push, num, &token.loc),
                _ => panic!("[ERROR]: TokenType::Int must have TokenValue::Str")
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<std::path::Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn step_col_while(line: &String, index: usize, until: impl Fn(u8) -> bool) -> usize {
    let mut i = index;
    while i < line.len() && until(line.as_bytes()[i]) {
        i += 1;
    }
    i
}
