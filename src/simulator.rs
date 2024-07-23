use crate::operation::{Operation, OperationKind};

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
        match &op.kind {
            OperationKind::PushInt(value) => {
                stack.push(*value);
                ip += 1;
            }
            OperationKind::PushChar(value) => {
                stack.push(*value as i32);
                ip += 1;
            }
            OperationKind::PushStr(value) => {
                stack.push(value.len() as i32);                 // push the count of u8 chars
                stack.push(string_space_counter as i32);        // push the start address
                memory.splice(string_space_counter.., value.bytes());
                string_space_counter += value.len();
                assert!(string_space_counter < STRING_SPACE, "[ERROR]: string space overflow");
                ip += 1;
            }
            /* -------------------------------- // Stack -------------------------------- */
            OperationKind::Dump => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <dump> 'dump' expects 1 operand", op.loc));
                println!("{a}");
                ip += 1;
            }
            OperationKind::Drop => {
                stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <drop> 'drop' expects 1 operand", op.loc));
                ip += 1;
            }
            OperationKind::Duplicate => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate> 'dup' expects 1 operand", op.loc));
                stack.push(a);
                stack.push(a);
                ip += 1;
            }
            OperationKind::Duplicate2 => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                stack.push(b);
                ip += 1;
            }
            OperationKind::Over => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(a);
                stack.push(b);
                stack.push(a);
                ip += 1;   
            }
            OperationKind::Swap => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <duplicate-2> 'dup2' expects 2 operands (second)", op.loc));
                stack.push(b);
                stack.push(a);
                ip += 1;     
            }
            /* ---------------------------------- // IO --------------------------------- */
            OperationKind::PrintChar => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <print> 'print' expects 1 operand (first)", op.loc));
                stack.push(a);
                println!("{}", u8::try_from(a).unwrap_or_else(|e| panic!("[ERROR]: 'print' may only be used with 'u8', instead {a} was provided: {e}")) as char);
                ip += 1;     
            }
        /* ------------------------------ // Arithmetic ----------------------------- */
            OperationKind::Add => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <plus> '+' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <plus> '+' expects 2 operands (second operand)", op.loc));
                stack.push(a + b);
                ip += 1;
            }
            OperationKind::Subtract => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <minus> '-' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <minus> '-' expects 2 operands (second operand)", op.loc));
                stack.push(a - b);
                ip += 1;
            }
            OperationKind::Multiply => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <multiply> '*' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <multiply> '*' expects 2 operands (second)", op.loc));
                stack.push((a * b) as i32);
                ip += 1;
            }
            OperationKind::Divide => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <divide> '/' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <divide> '/' expects 2 operands (second)", op.loc));
                stack.push((a / b) as i32);
                ip += 1;
            }
            OperationKind::Modulo => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push(a % b);
                ip += 1;
            }
        /* -------------------------------- // Logic -------------------------------- */
            OperationKind::Equal => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal> '=' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal> '=' expects 2 operands (second operand)", op.loc));
                stack.push((a == b) as i32);
                ip += 1;
            }
            OperationKind::NotEqual => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <not-equal> '!=' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <not-equal> '!=' expects 2 operands (second operand)", op.loc));
                stack.push((a != b) as i32);
                ip += 1;
            }
            OperationKind::Less => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <less> '<' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <less> '<' expects 2 operands (second operand)", op.loc));
                stack.push((a < b) as i32);
                ip += 1;
            }
            OperationKind::Greater => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <greater> '>' expects 2 operands (first operand)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <greater> '>' expects 2 operands (second operand)", op.loc));
                stack.push((a > b) as i32);
                ip += 1;
            }
            OperationKind::GreaterEqual => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-greater> '>=' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-greater> '>=' expects 2 operands (second)", op.loc));
                stack.push((a >= b) as i32);
                ip += 1;
            }
            OperationKind::LessEqual => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-less> '<=' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <equal-less> '<=' expects 2 operands (second)", op.loc));
                stack.push((a <= b) as i32);
                ip += 1;
            }
            OperationKind::Not => {
                let a = stack.pop().unwrap() != 0;
                stack.push((!a) as i32);
                ip += 1;
            }
        /* -------------------------------- // Block -------------------------------- */
            OperationKind::End(jump) => {
                if *jump >= 0 {
                    ip = *jump as usize;
                    continue;
                }
                ip += 1;
            }
            OperationKind::If(jump) => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <if-statement> 'if' expects 1 operand", op.loc)) != 0;
                if a == false {
                    ip = *jump as usize;
                    continue;
                }
                ip += 1;
            }
            OperationKind::Else(jump) => {
                ip = *jump as usize;
                ip += 1;
            }
            OperationKind::Do(jump) => {
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <do-statement> 'do' expects 1 operand", op.loc)) != 0;
                if a == false {
                    ip = *jump as usize;
                    continue;
                }
                ip += 1;
            }
            OperationKind::While => {
                ip += 1;
            }
            OperationKind::Macro => {
                unreachable!("macro should have been gone by now");
            }
            OperationKind::Include(file) => {
                panic!("include file not inplemeted: {file}");
            }
        /* ------------------------------- // Bitwise ------------------------------- */
            OperationKind::BitAnd => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <bit-and> '&' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <bit-and> '&' expects 2 operands (second)", op.loc));
                stack.push((a & b) as i32);
                ip += 1;
            }
            OperationKind::BitOr => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a | b) as i32);
                ip += 1;
            }
            OperationKind::ShiftRight => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a >> b) as i32);
                ip += 1;
            }
            OperationKind::ShiftLeft => {
                let b = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (first)", op.loc));
                let a = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <modulo> '%' expects 2 operands (second)", op.loc));
                stack.push((a << b) as i32);
                ip += 1;
            }
        /* -------------------------------- // Memory ------------------------------- */
            OperationKind::MemoryPush => {
                stack.push(0 + STRING_SPACE as i32); 
                ip += 1;
            }
            OperationKind::MemoryLoad => {
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <load> 'load' expects 1 operand", op.loc)) as usize;
                let value = memory[mem_index] as i32;
                stack.push(value);
                ip += 1;
            }
            OperationKind::MemoryStore => {
                let value = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <store> 'store' expects 2 operands (first)", op.loc)) as u8;
                let mem_index = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) <store> 'store' expects 2 operands (second)", op.loc)) as usize;
                memory[mem_index] = value;
                ip += 1;
            }
        /* ------------------------------- // Syscall ------------------------------- */
            OperationKind::Syscall1 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `syscall1` expects 2 operands (arg0)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `syscall1` expects 2 operands (arg1)", op.loc));
                match code {
                    60 => { // exit
                        if !SILENT {
                            println!("<sys_exit> ({arg1})");
                        }
                        break;
                    }
                    _ => panic!("[ERROR]: {} `syscall1` Unknown syscall with 2 args: {code}, {arg1}", op.loc)

                }
            }
            OperationKind::Syscall2 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg0)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg1)", op.loc));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg2)", op.loc));
                panic!("[ERROR]: {} `syscall2` Unknown syscall with 3 args: {code}, {arg1}, {arg2}", op.loc);
            }
            OperationKind::Syscall3 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `syscall3` expects 4 operands (arg0)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `syscall3` expects 4 operands (arg1)", op.loc));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `syscall3` expects 4 operands (arg2)", op.loc));
                let arg3 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `syscall3` expects 4 operands (arg3)", op.loc));
                match code {
                    1 => { // write
                        match arg1 { // file desc
                            1 | 2 => { // stdout and stderr
                                print!("{}", std::str::from_utf8(&memory[arg2 as usize..(arg3 + arg2) as usize]).unwrap());
                                ip += 1;
                            }
                            _ => panic!("[ERROR]: {} `syscall3` (syscall write) unknown file descriptor {arg1}", op.loc)
                        } 
                    }
                    _ => panic!("[ERROR]: {} `syscall3` Unknown syscall with 4 args: {code}, {arg1}, {arg2}, {arg3}", op.loc)
                }
            }            
            OperationKind::Syscall4 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg0)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg1)", op.loc));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg2)", op.loc));
                let arg3 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg3)", op.loc));
                let arg4 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall4` expects 5 operands (arg4)", op.loc));
                panic!("[ERROR]: {} `syscall4` Unknown syscall with 5 args: {code}, {arg1}, {arg2}, {arg3}, {arg4}", op.loc);
            }
            OperationKind::Syscall5 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg0)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg1)", op.loc));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg2)", op.loc));
                let arg3 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg3)", op.loc));
                let arg4 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg4)", op.loc));
                let arg5 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg5)", op.loc));
                panic!("[ERROR]: {} `syscall5` Unknown syscall with 6 args: {code}, {arg1}, {arg2}, {arg3}, {arg4}, {arg5}", op.loc);
            }
            OperationKind::Syscall6 => {
                let code = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg0)", op.loc));
                let arg1 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg1)", op.loc));
                let arg2 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg2)", op.loc));
                let arg3 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg3)", op.loc));
                let arg4 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg4)", op.loc));
                let arg5 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg5)", op.loc));
                let arg6 = stack.pop().unwrap_or_else(|| panic!("[ERROR]: {} (Empty Stack) `sycall5` expects 6 operands (arg6)", op.loc));
                panic!("[ERROR]: {} `syscall6` Unknown syscall with 7 args: {code}, {arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6}", op.loc);
            }
        }
    }
}
