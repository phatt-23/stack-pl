use crate::operation::{Operation, OperationType, OperationValue};

const SILENT: bool = true;
const STRING_SPACE: usize =  1_024;
const MEMORY_SPACE: usize = 64_000;

pub fn simulate_program(program: &Vec<Operation>) {
    let mut string_space_counter: usize = 0;

    let mut stack: Vec<i32> = Vec::new();
    let mut memory: Vec<u8> = vec![0; STRING_SPACE + MEMORY_SPACE];

    let mut ip: usize = 0;
    while ip < program.len() {
        let op = &program[ip];
        match (&op.op_type, &op.value) {
            (OperationType::PushInt, OperationValue::Int(value)) => {
                stack.push(*value);
                ip += 1;
            }
            (OperationType::PushStr, OperationValue::Str(value)) => {
                stack.push(value.len() as i32);                 // push the count of u8 chars
                stack.push(string_space_counter as i32);        // push the start address
                memory.splice(string_space_counter.., value.bytes());
                string_space_counter += value.len();
                assert!(string_space_counter < STRING_SPACE, "[ERROR]: string space overflow");
                ip += 1;
            }
            /* -------------------------------- // Stack -------------------------------- */
            (OperationType::Dump, OperationValue::Nothing) => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <dump> 'dump' expects 1 operand", op.loc));
                println!("{a}");
                ip += 1;
            }
            (OperationType::Drop, OperationValue::Nothing) => {
                stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <drop> 'drop' expects 1 operand", op.loc));
                ip += 1;
            }
            (OperationType::Duplicate, OperationValue::Nothing) => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate> 'dup' expects 1 operand", op.loc));
                stack.push(a);
                stack.push(a);
                ip += 1;
            }
            (OperationType::Duplicate2, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                stack.push(b);
                ip += 1;
            }
            (OperationType::Over, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                ip += 1;   
            }
            (OperationType::Swap, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(b);
                stack.push(a);
                ip += 1;     
            }
        /* ------------------------------ // Arithmetic ----------------------------- */
            (OperationType::Add, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <plus> '+' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <plus> '+' expects 2 operands (second operand)", op.loc));
                stack.push(a + b);
                ip += 1;
            }
            (OperationType::Subtract, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <minus> '-' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <minus> '-' expects 2 operands (second operand)", op.loc));
                stack.push(a - b);
                ip += 1;
            }
            (OperationType::Multiply, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <multiply> '*' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <multiply> '*' expects 2 operands (second)", op.loc));
                stack.push((a * b) as i32);
                ip += 1;
            }
            (OperationType::Divide, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <divide> '/' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <divide> '/' expects 2 operands (second)", op.loc));
                stack.push((a / b) as i32);
                ip += 1;
            }
            (OperationType::Modulo, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a % b) as i32);
                ip += 1;
            }
        /* -------------------------------- // Logic -------------------------------- */
            (OperationType::Equal, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal> '=' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal> '=' expects 2 operands (second operand)", op.loc));
                stack.push((a == b) as i32);
                ip += 1;
            }
            (OperationType::NotEqual, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <not-equal> '!=' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <not-equal> '!=' expects 2 operands (second operand)", op.loc));
                stack.push((a != b) as i32);
                ip += 1;
            }
            (OperationType::Less, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <less> '<' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <less> '<' expects 2 operands (second operand)", op.loc));
                stack.push((a < b) as i32);
                ip += 1;
            }
            (OperationType::Greater, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <greater> '>' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <greater> '>' expects 2 operands (second operand)", op.loc));
                stack.push((a > b) as i32);
                ip += 1;
            }
            (OperationType::GreaterEqual, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-greater> '>=' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-greater> '>=' expects 2 operands (second)", op.loc));
                stack.push((a >= b) as i32);
                ip += 1;
            }
            (OperationType::LessEqual, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-less> '<=' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-less> '<=' expects 2 operands (second)", op.loc));
                stack.push((a <= b) as i32);
                ip += 1;
            }
            (OperationType::Not, OperationValue::Nothing) => {
                let a = stack.pop().unwrap() != 0;
                stack.push((!a) as i32);
                ip += 1;
            }
        /* -------------------------------- // Block -------------------------------- */
            (OperationType::End, OperationValue::Nothing) => {
                if op.jump >= 0 {
                    ip = op.jump as usize;
                    continue;
                }
                ip += 1;
            }
            (OperationType::If, OperationValue::Nothing) => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <if-statement> 'if' expects 1 operand", op.loc)) != 0;
                if a == false {
                    ip = op.jump as usize;
                    continue;
                }
                ip += 1;
            }
            (OperationType::Else, OperationValue::Nothing) => {
                ip = op.jump as usize;
                ip += 1;
            }
            (OperationType::Do, OperationValue::Nothing) => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <do-statement> 'do' expects 1 operand", op.loc)) != 0;
                if a == false {
                    ip = op.jump as usize;
                    continue;
                }
                ip += 1;
            }
            (OperationType::While, OperationValue::Nothing) => {
                ip += 1;
            }
        /* ------------------------------- // Bitwise ------------------------------- */
            (OperationType::BitAnd, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <bit-and> '&' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <bit-and> '&' expects 2 operands (second)", op.loc));
                stack.push((a & b) as i32);
                ip += 1;
            }
            (OperationType::BitOr, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a | b) as i32);
                ip += 1;
            }
            (OperationType::ShiftRight, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a >> b) as i32);
                ip += 1;
            }
            (OperationType::ShiftLeft, OperationValue::Nothing) => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a << b) as i32);
                ip += 1;
            }
        /* -------------------------------- // Memory ------------------------------- */
            (OperationType::MemoryPush, OperationValue::Nothing) => {
                stack.push(0 + STRING_SPACE as i32); 
                ip += 1;
            }
            (OperationType::MemoryLoad, OperationValue::Nothing) => {
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <load> 'load' expects 1 operand", op.loc)) as usize;
                let value = memory[mem_index] as i32;
                stack.push(value);
                ip += 1;
            }
            (OperationType::MemoryStore, OperationValue::Nothing) => {
                let value = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <store> 'store' expects 2 operands (first)", op.loc)) as u8;
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <store> 'store' expects 2 operands (second)", op.loc)) as usize;
                memory[mem_index] = value;
                ip += 1;
            }
        /* ------------------------------- // Syscall ------------------------------- */
            (OperationType::Syscall1, OperationValue::Nothing) => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall1> 'syscall1' expects 2 operands (first)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall1> 'syscall1' expects 2 operands (second)", op.loc));
                match code {
                    60 => { // exit
                        if !SILENT {
                            println!("<sys_exit> ({arg1})");
                        }
                        break;
                    }
                    _ => panic!("[ERROR]: {} <syscall1> Unknown syscall with 2 args", op.loc)
                }
            }
            (OperationType::Syscall2, OperationValue::Nothing) => {
                todo!();
            }
            (OperationType::Syscall3 , OperationValue::Nothing) => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'syscall3' expects 4 operands (first)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'syscall3' expects 4 operands (second)", op.loc));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'syscall3' expects 4 operands (third)", op.loc));
                let arg3 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <syscall3> 'syscall3' expects 4 operands (fourth)", op.loc));
                match code {
                    1 => { // write
                        match arg1 { // file desc
                            1 | 2 => { // stdout and stderr
                                print!("{}", std::str::from_utf8(&memory[arg2 as usize..(arg3 + arg2) as usize]).unwrap());
                                ip += 1;
                            }
                            _ => panic!("[ERROR]: {} <syscall3> (syscall write) unknown file descriptor {arg1}", op.loc)
                        } 
                    }
                    _ => panic!("[ERROR]: {} <syscall3> Unknown syscall with 4 args", op.loc)
                }
            }            
            (OperationType::Syscall4, OperationValue::Nothing) => {
                todo!();
            }
            (OperationType::Syscall5, OperationValue::Nothing) => {
                todo!();
            }
            (OperationType::Syscall6, OperationValue::Nothing) => {
                todo!();
            }
            (OperationType::Macro, _) => {
                println!("refactor operation types and values together you fuck");
            }
            (op_type, op_value) => panic!("Unexpected OperationType and OperationValue combination: type: {:?}, value: {:?}", op_type, op_value)
        }
    }
}
