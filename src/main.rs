use std::{env , str};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod operation;
use operation::Operation;
pub mod token;
use token::Token;
pub mod compiler;
pub mod simulator;

fn print_usage(args: Vec<String>) {
    println!("[ERROR]: Usage: {} <com|sim> <file>", &args[0].as_str());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
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

fn lex_line(program: &mut Vec<Operation>, 
            ip: usize, 
            filepath: &String, 
            row: usize, 
            line: String) -> usize 
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
            "dump" => program.push(operation::op_dump(ip)),
            "dup" => program.push(operation::op_dup(ip)),
            "+" => program.push(operation::op_plus(ip)),
            "-" => program.push(operation::op_minus(ip)),
            "=" => program.push(operation::op_eq(ip)),
            "<" => program.push(operation::op_le(ip)),
            ">" => program.push(operation::op_gr(ip)),
            "<=" => program.push(operation::op_eqle(ip)),
            ">=" => program.push(operation::op_eqgr(ip)),
            "!" | "not" => program.push(operation::op_not(ip)),
            "end" => program.push(operation::op_end(ip)),
            "if" => program.push(operation::op_if(ip)),
            "else" => program.push(operation::op_else(ip)),
            "while" => program.push(operation::op_while(ip)),
            "do" => program.push(operation::op_do(ip)),
            number => {
                match number.parse::<i64>() {
                    Ok(v) => program.push(operation::op_push(ip, v)),
                    Err(e) => {
                        println!("[ERROR] Syntax: {}:{}:{}:, {:?}, {:?}", filepath, row + 1, col + 1, w, e);
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

fn lex_file(filepath: &str, program: &mut Vec<Operation>) {
    if let Ok(lines) = read_lines(filepath) {
        println!("[INFO]: reading the program from '{}'", filepath);
        let mut ip_counter = 0;
        for (row, line) in lines.enumerate() {
            match line {
                Ok(l) => ip_counter = lex_line(program, ip_counter, &String::from(filepath), row , l),
                Err(e) => panic!("{}", e),
            }
        }
        crossreference_blocks(program);
    }
}

fn crossreference_blocks(program: &mut Vec<Operation>) {
    let mut stack: Vec<usize> = Vec::new();
        
    for ip in 0..program.len() {
        match program[ip].token {
            Token::If => stack.push(ip),
            Token::While => stack.push(ip),
            Token::Do => {
                let while_index = stack.pop().unwrap();
                let while_op = program[while_index];
                program[ip].value = while_op.index as i64;
                stack.push(ip);
            }
            Token::Else => {
                let if_index = stack.pop().unwrap();
                program[if_index].value = (ip + 1) as i64; 
                stack.push(ip);
            }
            Token::End => {
                let end_op = program[ip];
                let prev_index = stack.pop().unwrap();
                let prev_op = program[prev_index];
                if prev_op.token == Token::If || prev_op.token == Token::Else {
                    program[prev_index].value = end_op.index as i64;
                }
                if prev_op.token == Token::Do {
                    program[ip].value = prev_op.value as i64;
                    program[prev_index].value = (end_op.index + 1) as i64;
                }
            }
            // ignore other instructions
            Token::Push 
                | Token::Dump 
                | Token::Dup 
                | Token::Plus 
                | Token::Minus 
                | Token::Eq 
                | Token::Le 
                | Token::Gr 
                | Token::EqGr 
                | Token::EqLe 
                | Token::Not 
                => {} 
        }
    }
}

fn main() {
    // check the command line arguments
    let args: Vec<String> = env::args().collect();
    if env::args().count() < 2 {
        print_usage(args);
        return;
    }

    let mut program: Vec<Operation> = Vec::new(); // compose the program from given file
    // open the file
    let input_filepath = args[2].as_str();
    lex_file(input_filepath, &mut program);
    for op in &program {
        println!("[INFO]: {:?}", op);
    }

    // simulate or compile the file
    match args[1].as_str() {
        "sim" => simulator::simulate_program(program),
        "com" => {
            compiler::create_assembly(program, "output.asm").unwrap();
            compiler::compile_assembly();
        }
        _ => print_usage(args),
    }
}



