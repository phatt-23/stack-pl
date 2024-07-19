use std::{env , str};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

pub mod operation;
use operation::Operation;
pub mod token;
use token::Token;
pub mod compiler;


fn simulate_program(program: Vec<Operation>) {
    println!("[INFO]: Simulating the program");
    let mut stack: Vec<i64> = Vec::new();

    for op in program {
        match op.token {
            Token::Push => stack.push(op.value),
            Token::Dump => {
                let v = stack.pop();
                println!("{}", v.unwrap());
            }
            Token::Plus => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Minus => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            Token::Eq => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a == b) as i64);
            }
            Token::Le => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a < b) as i64);
            }
            Token::Gr => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a > b) as i64);
            }
            Token::End => {
                todo!();
            }
            Token::If => {
                todo!();
            }
            Token::Else => {
                todo!();
            }
        }
    }
}

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
            "+" => program.push(operation::op_plus(ip)),
            "-" => program.push(operation::op_minus(ip)),
            "dump" => program.push(operation::op_dump(ip)),
            "=" => program.push(operation::op_eq(ip)),
            "<" => program.push(operation::op_le(ip)),
            ">" => program.push(operation::op_gr(ip)),
            "end" => program.push(operation::op_end(ip)),
            "if" => program.push(operation::op_if(ip)),
            "else" => program.push(operation::op_else(ip)),
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
                Ok(l) => {
                    ip_counter = lex_line(program, ip_counter, &String::from(filepath), row , l);
                }
                Err(e) => panic!("{}", e),
            }
        }
        crossreference_blocks(program);
    }
}

fn crossreference_blocks(program: &mut Vec<Operation>) {
    let mut stack: Vec<usize> = Vec::new();
    
    for i in 0..program.len() {
        match program[i].token {
            Token::End => {
                let if_else_ip = match stack.pop() {
                    Some(ip) => ip,
                    None => {println!("[ERROR]: 'End' token found without matching 'if' or 'else'"); return;}
                };
                let end_index = i;
                let if_or_else_token = program[if_else_ip].token;
                if if_or_else_token == Token::If || if_or_else_token == Token::Else {
                    program[if_else_ip].value = end_index as i64;
                }
                if if_or_else_token == Token::Else {
                    let if_index = match stack.pop() {
                        Some(index) => index,
                        None => {println!("[ERROR]: 'else' operation found without preceding 'if'"); return;}
                    };
                    if program[if_index].token == Token::If {
                        program[if_index].value = program[if_else_ip].index as i64 + 1;
                    }
                }
            }
            Token::If | Token::Else => stack.push(i),
            _ => {} // ignore other instructions
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
        "sim" => simulate_program(program),
        "com" => {
            compiler::compile_program(program, "output.asm").unwrap();
            print_command_output( Command::new("nasm").arg("-felf64").arg("output.asm").output().expect("nasm failed") );
            print_command_output( Command::new("ld").arg("output.o").arg("-o").arg("program").output().expect("ld failed") );
        }
        _ => print_usage(args),
    }
}


fn print_command_output(output: std::process::Output) {
    if !&output.stdout.is_empty() {
        println!("[INFO]: stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
    if !&output.stderr.is_empty() {
        println!("[ERROR]: stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        println!("[ERROR]: status:\n{}", output.status);
    }
}
