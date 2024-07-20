use std::fs::File;
use std::io::{self, BufRead};
use super::operation::{self, Operation, Token};

pub fn lex_file(filepath: &str, program: &mut Vec<Operation>) {
    if let Ok(lines) = read_lines(filepath) {
        println!("[INFO]: reading the program from '{}'", filepath);
        let mut ip_counter = 0;
        for (row, line) in lines.enumerate() {
            match line {
                Ok(l) => ip_counter = lex_line(program, ip_counter, &String::from(filepath), row + 1 , l),
                Err(e) => panic!("{}", e),
            }
        }
        crossreference_blocks(program);
    }
}

pub fn lex_line(program: &mut Vec<Operation>, ip: usize, filepath: &String, row: usize, line: String) -> usize 
{
    println!("[INFO]: Raw line ({row}): {:?}", line);
    let inter: Vec<&str> = line.split("//").take(1).collect();
    let words: Vec<&str> = inter.iter().flat_map(|&s| s.split_whitespace()).collect();
    println!("[INFO]: Splitted ({row}): {:?}", words);

    let mut ip = ip;
    let mut col = step_col(&line, 0, |x| x != b' ');
    for w in words {
        col = step_col(&line, col, |x| x != b' ');
        let mut advance_index = true;
        match w {
            "dump" => program.push(operation::op_dump(ip, filepath, row, col)),
            "dup"  => program.push(operation::op_dup(ip, filepath, row, col)),
            "dup2" => program.push(operation::op_dup_2(ip, filepath, row, col)),
            "drop" => program.push(operation::op_drop(ip, filepath, row, col)),
            "swap" => program.push(operation::op_swap(ip, filepath, row, col)),
            "over" => program.push(operation::op_over(ip, filepath, row, col)),

            "+" | "add" => program.push(operation::op_plus(ip, filepath, row, col)),
            "-" | "sub" => program.push(operation::op_minus(ip, filepath, row, col)),
            "*" | "mul" => program.push(operation::op_multiply(ip, filepath, row, col)),
            "/" | "div" => program.push(operation::op_divide(ip, filepath, row, col)),
            "%" | "mod" => program.push(operation::op_modulo(ip, filepath, row, col)),
            
            "="  | "eq"  => program.push(operation::op_eq(ip, filepath, row, col)),
            "!=" | "neq" => program.push(operation::op_not_eq(ip, filepath, row, col)),
            "<"  | "le"  => program.push(operation::op_le(ip, filepath, row, col)),
            ">"  | "gr"  => program.push(operation::op_gr(ip, filepath, row, col)),
            "<=" | "eql" => program.push(operation::op_eq_le(ip, filepath, row, col)),
            ">=" | "egr" => program.push(operation::op_eq_gr(ip, filepath, row, col)),
            "!"  | "not" => program.push(operation::op_not(ip, filepath, row, col)),
            
            "<<" | "shl"  => program.push(operation::op_shift_left(ip, filepath, row, col)),
            ">>" | "shr"  => program.push(operation::op_shift_right(ip, filepath, row, col)),
            "&"  | "band" => program.push(operation::op_bit_and(ip, filepath, row, col)),
            "|"  | "bor"  => program.push(operation::op_bit_or(ip, filepath, row, col)),
            
            "end"   => program.push(operation::op_end(ip, filepath, row, col)),
            "if"    => program.push(operation::op_if(ip, filepath, row, col)),
            "else"  => program.push(operation::op_else(ip, filepath, row, col)),
            "while" => program.push(operation::op_while(ip, filepath, row, col)),
            "do"    => program.push(operation::op_do(ip, filepath, row, col)),
            
            "mem"           => program.push(operation::op_memory(ip, filepath, row, col)),
            ","   | "load"  => program.push(operation::op_load(ip, filepath, row, col)),
            "."   | "store" => program.push(operation::op_store(ip, filepath, row, col)),
            
            "syscall1" => program.push(operation::op_syscall_1(ip, filepath, row, col)),
            "syscall3" => program.push(operation::op_syscall_3(ip, filepath, row, col)),
            
            number => {
                match number.parse::<i64>() {
                    Ok(v) => program.push(operation::op_push(ip, v, filepath, row, col)),
                    Err(e) => {
                        println!("[ERROR] Unknown Token: {}:{}:{}:, {:?}, {:?}", filepath, row + 1, col + 1, w, e);
                        advance_index = false;
                    }
                }
            }
        }
        col = step_col(&line, col, |x| x == b' ');
        if advance_index {
            ip += 1
        }
    }
    ip
}

fn crossreference_blocks(program: &mut Vec<Operation>) {
    let mut stack: Vec<usize> = Vec::new();
        
    for ip in 0..program.len() {
        match program[ip].token {
            Token::If => stack.push(ip),
            Token::While => stack.push(ip),
            Token::Do => {
                let while_index = stack.pop().unwrap();
                let while_op = &program[while_index];
                program[ip].value = while_op.index as i64;
                stack.push(ip);
            }
            Token::Else => {
                let if_index = stack.pop().unwrap();
                program[if_index].value = (ip + 1) as i64; 
                stack.push(ip);
            }
            Token::End => {
                let prev_index = stack.pop().unwrap();
                if program[prev_index].token == Token::If || program[prev_index].token == Token::Else {
                    program[prev_index].value =  program[ip].index as i64;
                }
                if program[prev_index].token == Token::Do {
                    program[ip].value = program[prev_index].value as i64;
                    program[prev_index].value = ( program[ip].index + 1) as i64;
                }
            }
            // ignore other instructions
            Token::Push 
                | Token::Dump | Token::Drop
                | Token::Swap | Token::Over
                | Token::Dup | Token::Dup2
                | Token::Plus | Token::Minus 
                | Token::Multiply | Token::Divide
                | Token::Modulo
                | Token::Eq | Token::NotEq | Token::Not 
                | Token::Le | Token::Gr 
                | Token::EqGr | Token::EqLe
                | Token::BitAnd | Token::BitOr 
                | Token::ShiftRight | Token::ShiftLeft 
                | Token::Memory
                | Token::Load
                | Token::Store
                | Token::Syscall1 
                | Token::Syscall3
                => {} 
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
