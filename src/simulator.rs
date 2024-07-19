use super::Token;
use super::operation::Operation;

const MEMORY_SIZE: usize = 640_000;

pub fn simulate_program(program: &Vec<Operation>) {
    println!("[INFO]: Simulating the program");
    let mut stack: Vec<i64> = Vec::new();
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

    let mut ip: usize = 0;
    while ip < program.len() {
        let op = program[ip];
        match op.token {
            Token::Push => {
                stack.push(op.value);
            }
            Token::Dump => {
                println!("{}", stack.pop().expect("[ERROR]: (Empty Stack) <dump> 'dump' expects 1 operand"));
            }
            Token::Plus => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <plus> '+' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <plus> '+' expects 2 operands (second operand)");
                stack.push(a + b);
            }
            Token::Minus => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <minus> '-' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <minus> '-' expects 2 operands (second operand)");
                stack.push(a - b);
            }
            Token::Eq => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (second operand)");
                stack.push((a == b) as i64);
            }
            Token::NotEq => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (second operand)");
                stack.push((a != b) as i64);
            }
            Token::Le => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <less> '<' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <less> '<' expects 2 operands (second operand)");
                stack.push((a < b) as i64);
            }
            Token::Gr => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <greater> '>' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <greater> '>' expects 2 operands (second operand)");
                stack.push((a > b) as i64);
            }
            Token::End => {
                if op.value >= 0 {
                    ip = (op.value - 1) as usize;
                }
            }
            Token::If => {
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <if-statement> 'if' expects 1 operand") != 0;
                if a == false {
                    ip = (op.value + 1) as usize;
                }
            }
            Token::Else => {
                ip = op.value as usize;
            }
            Token::Dup => {
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <duplicate> 'dup' expects 1 operand");
                stack.push(a);
                stack.push(a);
            }
            Token::Do => {
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <do-statement> 'do' expects 1 operand") != 0;
                if a == false {
                    ip = (op.value) as usize;
                }
            }
            Token::While => {
                // nothing
            }
            Token::EqGr => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal-greater> '>=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal-greater> '>=' expects 2 operands (second operand)");
                stack.push((a >= b) as i64);
            }
            Token::EqLe => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal-less> '<=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal-less> '<=' expects 2 operands (second operand)");
                stack.push((a <= b) as i64);
            }
            Token::Not => {
                let a = stack.pop().unwrap() != 0;
                stack.push((!a) as i64);
            }
            Token::Multiply => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <multiply> '*' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <multiply> '*' expects 2 operands (second operand)");
                stack.push((a * b) as i64);
            }
            Token::Divide => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <divide> '/' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <divide> '/' expects 2 operands (second operand)");
                stack.push((a / b) as i64);
            }
            Token::Modulo => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <modulo> '%' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <modulo> '%' expects 2 operands (second operand)");
                stack.push((a % b) as i64);
            }
            Token::Memory => {
                stack.push((0) as i64); 
            }
            Token::Load => {
                let mem_index = stack.pop().expect("[ERROR]: (Empty Stack) <load> 'load' expects 1 operand") as usize;
                let value = memory[mem_index] as i64;
                stack.push(value);
            }
            Token::Store => {
                let value = stack.pop().expect("[ERROR]: (Empty Stack) <store> 'store' expects 2 operands (first operand)") as u8;
                let mem_index = stack.pop().expect("[ERROR]: (Empty Stack) <store> 'store' expects 2 operands (second operand)") as usize;
                memory[mem_index] = value;
            }
            Token::Syscall1 => {
                let code = stack.pop().expect("[ERROR]: (Empty Stack) <syscall1> 'store' expects 2 operands (first operand)");
                let arg1 = stack.pop().expect("[ERROR]: (Empty Stack) <syscall1> 'store' expects 2 operands (second operand)");
                match code {
                    60 => { // exit
                        println!("<syscall> exited with status ({arg1})");
                        break;
                    }
                    _ => println!("[ERROR]: <syscall1> Unknown syscall with 2 args")
                }
            }
            Token::Syscall3  => {
                let code = stack.pop().expect("[ERROR]: (Empty Stack) <syscall3> 'store' expects 4 operands (first operand)");
                let arg1 = stack.pop().expect("[ERROR]: (Empty Stack) <syscall3> 'store' expects 4 operands (second operand)");
                let arg2 = stack.pop().expect("[ERROR]: (Empty Stack) <syscall3> 'store' expects 4 operands (first operand)");
                let arg3 = stack.pop().expect("[ERROR]: (Empty Stack) <syscall3> 'store' expects 4 operands (first operand)");
                match code {
                    1 => { // write
                        match arg1 { // file desc
                            1 => { // stdout
                                for i in 0..arg3 + 1 {
                                    print!("{}", memory[(arg2 + i) as usize] as char);
                                }
                            }
                            2 => { // stderr
                                for i in 0..arg3 + 1 {
                                    print!("{}", memory[(arg2 + i) as usize] as char);
                                }
                            }
                            _ => println!("[ERROR]: <syscall3> (syscall write) unknown file descriptor {arg1}")
                        } 
                    }
                    _ => println!("[ERROR]: <syscall3> Unknown syscall with 4 args")
                }
            }
        }
        ip += 1;
    }
}
