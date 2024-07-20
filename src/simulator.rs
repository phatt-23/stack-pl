use super::operation::{Operation, Token};

const MEMORY_SIZE: usize = 640_000;

pub fn simulate_program(program: &Vec<Operation>) {
    println!("[INFO]: Simulating the program");
    let mut stack: Vec<i64> = Vec::new();
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

    let mut ip: usize = 0;
    while ip < program.len() {
        let op = &program[ip];
        match op.token {
            Token::Push => {
                stack.push(op.value);
                ip += 1;
            }
            Token::Dump => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <dump> 'dump' expects 1 operand", op.file, op.row, op.col));
                println!("{a}");
                ip += 1;
            }
            Token::Drop => {
                stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <drop> 'drop' expects 1 operand", op.file, op.row, op.col));
                ip += 1;
            }
            Token::Dup => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <duplicate> 'dup' expects 1 operand", op.file, op.row, op.col));
                stack.push(a);
                stack.push(a);
                ip += 1;
            }
            Token::Dup2 => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                stack.push(b);
                ip += 1;
            }
            Token::Over => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                ip += 1;   
            }
            Token::Swap => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push(b);
                stack.push(a);
                ip += 1;     
            }
            Token::Plus => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <plus> '+' expects 2 operands (first operand)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <plus> '+' expects 2 operands (second operand)", op.file, op.row, op.col));
                stack.push(a + b);
                ip += 1;
            }
            Token::Minus => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <minus> '-' expects 2 operands (first operand)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <minus> '-' expects 2 operands (second operand)", op.file, op.row, op.col));
                stack.push(a - b);
                ip += 1;
            }
            Token::Eq => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <equal> '=' expects 2 operands (first operand)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <equal> '=' expects 2 operands (second operand)", op.file, op.row, op.col));
                stack.push((a == b) as i64);
                ip += 1;
            }
            Token::NotEq => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <not-equal> '!=' expects 2 operands (first operand)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <not-equal> '!=' expects 2 operands (second operand)", op.file, op.row, op.col));
                stack.push((a != b) as i64);
                ip += 1;
            }
            Token::Le => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <less> '<' expects 2 operands (first operand)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <less> '<' expects 2 operands (second operand)", op.file, op.row, op.col));
                stack.push((a < b) as i64);
                ip += 1;
            }
            Token::Gr => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <greater> '>' expects 2 operands (first operand)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <greater> '>' expects 2 operands (second operand)", op.file, op.row, op.col));
                stack.push((a > b) as i64);
                ip += 1;
            }
            Token::End => {
                if op.value >= 0 {
                    ip = op.value as usize;
                    continue;
                }
                ip += 1;
            }
            Token::If => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <if-statement> 'if' expects 1 operand", op.file, op.row, op.col)) != 0;
                if a == false {
                    ip = op.value as usize;
                    continue;
                }
                ip += 1;
            }
            Token::Else => {
                ip = op.value as usize;
                ip += 1;
            }
            Token::Do => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <do-statement> 'do' expects 1 operand", op.file, op.row, op.col)) != 0;
                if a == false {
                    ip = op.value as usize;
                    continue;
                }
                ip += 1;
            }
            Token::While => {
                // nothing
                ip += 1;
            }
            Token::EqGr => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <equal-greater> '>=' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <equal-greater> '>=' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a >= b) as i64);
                ip += 1;
            }
            Token::EqLe => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <equal-less> '<=' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <equal-less> '<=' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a <= b) as i64);
                ip += 1;
            }
            Token::Not => {
                let a = stack.pop().unwrap() != 0;
                stack.push((!a) as i64);
                ip += 1;
            }
            Token::Multiply => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <multiply> '*' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <multiply> '*' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a * b) as i64);
                ip += 1;
            }
            Token::Divide => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <divide> '/' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <divide> '/' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a / b) as i64);
                ip += 1;
            }
            Token::Modulo => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a % b) as i64);
                ip += 1;
            }
            Token::BitAnd => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <bit-and> '&' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <bit-and> '&' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a & b) as i64);
                ip += 1;
            }
            Token::BitOr => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a | b) as i64);
                ip += 1;
            }
            Token::ShiftRight => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a >> b) as i64);
                ip += 1;
            }
            Token::ShiftLeft => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (first)", op.file, op.row, op.col));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <modulo> '%' expects 2 operands (second)", op.file, op.row, op.col));
                stack.push((a << b) as i64);
                ip += 1;
            }
            Token::Memory => {
                stack.push(0); 
                ip += 1;
            }
            Token::Load => {
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <load> 'load' expects 1 operand", op.file, op.row, op.col)) as usize;
                let value = memory[mem_index] as i64;
                stack.push(value);
                ip += 1;
            }
            Token::Store => {
                let value = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <store> 'store' expects 2 operands (first)", op.file, op.row, op.col)) as u8;
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <store> 'store' expects 2 operands (second)", op.file, op.row, op.col)) as usize;
                memory[mem_index] = value;
                ip += 1;
            }
            Token::Syscall1 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <syscall1> 'store' expects 2 operands (first)", op.file, op.row, op.col));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <syscall1> 'store' expects 2 operands (second)", op.file, op.row, op.col));
                match code {
                    60 => { // exit
                        println!("<syscall> exit ({arg1})");
                        break;
                    }
                    _ => panic!("[ERROR]: {}:{}:{}: <syscall1> Unknown syscall with 2 args", op.file, op.row, op.col)
                }
            }
            Token::Syscall3  => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <syscall3> 'store' expects 4 operands (first)", op.file, op.row, op.col));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <syscall3> 'store' expects 4 operands (second)", op.file, op.row, op.col));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <syscall3> 'store' expects 4 operands (third)", op.file, op.row, op.col));
                let arg3 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {}:{}:{}: (Empty Stack) <syscall3> 'store' expects 4 operands (fourth)", op.file, op.row, op.col));
                match code {
                    1 => { // write
                        match arg1 { // file desc
                            1 => { // stdout
                                for i in 0..arg3 + 1 {
                                    print!("{}", memory[(arg2 + i) as usize] as char);
                                }
                                ip += 1;
                            }
                            2 => { // stderr
                                for i in 0..arg3 + 1 {
                                    print!("{}", memory[(arg2 + i) as usize] as char);
                                }
                                ip += 1;
                            }
                            _ => panic!("[ERROR]: {}:{}:{}: <syscall3> (syscall write) unknown file descriptor {arg1}", op.file, op.row, op.col)
                        } 
                    }
                    _ => panic!("[ERROR]: {}:{}:{}: <syscall3> Unknown syscall with 4 args", op.file, op.row, op.col)
                }
            }
        }
    }
}
