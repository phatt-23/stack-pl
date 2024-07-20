use super::operation::{Operation, OperationType};

const MEMORY_SIZE: usize = 640_000;

pub fn simulate_program(program: &Vec<Operation>) {
    println!("[INFO]: Simulating the program");
    let mut stack: Vec<i64> = Vec::new();
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

    let mut ip: usize = 0;
    while ip < program.len() {
        let op = &program[ip];
        match op.op_type {
        /* -------------------------------- // Stack -------------------------------- */
            OperationType::Push => {
                stack.push(op.value);
                ip += 1;
            }
            OperationType::Dump => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <dump> 'dump' expects 1 operand", op.loc));
                println!("{a}");
                ip += 1;
            }
            OperationType::Drop => {
                stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <drop> 'drop' expects 1 operand", op.loc));
                ip += 1;
            }
            OperationType::Duplicate => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate> 'dup' expects 1 operand", op.loc));
                stack.push(a);
                stack.push(a);
                ip += 1;
            }
            OperationType::Duplicate2 => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                stack.push(b);
                ip += 1;
            }
            OperationType::Over => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                ip += 1;   
            }
            OperationType::Swap => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(b);
                stack.push(a);
                ip += 1;     
            }
        /* ------------------------------ // Arithmetic ----------------------------- */
            OperationType::Add => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <plus> '+' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <plus> '+' expects 2 operands (second operand)", op.loc));
                stack.push(a + b);
                ip += 1;
            }
            OperationType::Subtract => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <minus> '-' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <minus> '-' expects 2 operands (second operand)", op.loc));
                stack.push(a - b);
                ip += 1;
            }
            OperationType::Multiply => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <multiply> '*' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <multiply> '*' expects 2 operands (second)", op.loc));
                stack.push((a * b) as i64);
                ip += 1;
            }
            OperationType::Divide => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <divide> '/' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <divide> '/' expects 2 operands (second)", op.loc));
                stack.push((a / b) as i64);
                ip += 1;
            }
            OperationType::Modulo => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a % b) as i64);
                ip += 1;
            }
        /* -------------------------------- // Logic -------------------------------- */
            OperationType::Equal => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal> '=' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal> '=' expects 2 operands (second operand)", op.loc));
                stack.push((a == b) as i64);
                ip += 1;
            }
            OperationType::NotEqual => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <not-equal> '!=' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <not-equal> '!=' expects 2 operands (second operand)", op.loc));
                stack.push((a != b) as i64);
                ip += 1;
            }
            OperationType::Less => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <less> '<' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <less> '<' expects 2 operands (second operand)", op.loc));
                stack.push((a < b) as i64);
                ip += 1;
            }
            OperationType::Greater => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <greater> '>' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <greater> '>' expects 2 operands (second operand)", op.loc));
                stack.push((a > b) as i64);
                ip += 1;
            }
            OperationType::GreaterEqual => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-greater> '>=' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-greater> '>=' expects 2 operands (second)", op.loc));
                stack.push((a >= b) as i64);
                ip += 1;
            }
            OperationType::LessEqual => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-less> '<=' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-less> '<=' expects 2 operands (second)", op.loc));
                stack.push((a <= b) as i64);
                ip += 1;
            }
            OperationType::Not => {
                let a = stack.pop().unwrap() != 0;
                stack.push((!a) as i64);
                ip += 1;
            }
        /* -------------------------------- // Block -------------------------------- */
            OperationType::End => {
                if op.value >= 0 {
                    ip = op.value as usize;
                    continue;
                }
                ip += 1;
            }
            OperationType::If => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <if-statement> 'if' expects 1 operand", op.loc)) != 0;
                if a == false {
                    ip = op.value as usize;
                    continue;
                }
                ip += 1;
            }
            OperationType::Else => {
                ip = op.value as usize;
                ip += 1;
            }
            OperationType::Do => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <do-statement> 'do' expects 1 operand", op.loc)) != 0;
                if a == false {
                    ip = op.value as usize;
                    continue;
                }
                ip += 1;
            }
            OperationType::While => {
                ip += 1;
            }
        /* ------------------------------- // Bitwise ------------------------------- */
            OperationType::BitAnd => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <bit-and> '&' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <bit-and> '&' expects 2 operands (second)", op.loc));
                stack.push((a & b) as i64);
                ip += 1;
            }
            OperationType::BitOr => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a | b) as i64);
                ip += 1;
            }
            OperationType::ShiftRight => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a >> b) as i64);
                ip += 1;
            }
            OperationType::ShiftLeft => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a << b) as i64);
                ip += 1;
            }
        /* -------------------------------- // Memory ------------------------------- */
            OperationType::MemoryPush => {
                stack.push(0); 
                ip += 1;
            }
            OperationType::MemoryLoad => {
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <load> 'load' expects 1 operand", op.loc)) as usize;
                let value = memory[mem_index] as i64;
                stack.push(value);
                ip += 1;
            }
            OperationType::MemoryStore => {
                let value = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <store> 'store' expects 2 operands (first)", op.loc)) as u8;
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <store> 'store' expects 2 operands (second)", op.loc)) as usize;
                memory[mem_index] = value;
                ip += 1;
            }
        /* ------------------------------- // Syscall ------------------------------- */
            OperationType::Syscall1 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall1> 'store' expects 2 operands (first)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall1> 'store' expects 2 operands (second)", op.loc));
                match code {
                    60 => { // exit
                        println!("<syscall> exit ({arg1})");
                        break;
                    }
                    _ => panic!("[ERROR]: {} <syscall1> Unknown syscall with 2 args", op.loc)
                }
            }
            OperationType::Syscall3  => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'store' expects 4 operands (first)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'store' expects 4 operands (second)", op.loc));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'store' expects 4 operands (third)", op.loc));
                let arg3 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'store' expects 4 operands (fourth)", op.loc));
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
                            _ => panic!("[ERROR]: {} <syscall3> (syscall write) unknown file descriptor {arg1}", op.loc)
                        } 
                    }
                    _ => panic!("[ERROR]: {} <syscall3> Unknown syscall with 4 args", op.loc)
                }
            }
        }
    }
}
