mod subprogram;

use std::fs::File;
use std::io::{self, Write};
use crate::operation::{Operation, OperationKind};
use crate::intrinsic::IntrinsicType;

const MEMORY_SPACE: usize = 64_000;

pub fn generate_linux_nasm_x86_64(program: &Vec<Operation>, output: &str) -> Result<i32, std::io::Error> {
    let mut file = File::create(output).unwrap_or_else(|e| panic!("[ERROR]: Assembly file {output:?} creation failed! {e}"));
    writeln!(file, "[bits 64]")?;
    writeln!(file, ";;;")?;
    writeln!(file, "segment .text")?;
    writeln!(file, "        global _start")?;
    writeln!(file, "_start:")?;
    
    let mut strings: Vec<String> = Vec::new();
    for op in program {
        generate_operation(&mut file, op, &mut strings)?;
    }
    
    writeln!(file, "    ;;return")?;
    writeln!(file, "        mov rax, 60")?;
    writeln!(file, "        mov rdi, 0")?;
    writeln!(file, "        syscall")?;
    writeln!(file, ";;;_start")?;
    writeln!(file, "segment .data")?;

    for (index, string) in strings.iter().enumerate() {
        let b: Vec<_> = string.as_bytes().iter()
            .map(|x| format!("0x{x:02X}"))
            .collect();
        let bytes = format!("{:?}", b)
            .replace(|c| c == '[' || c == ']' || c == '"', "");
        writeln!(file, "        STRING_{index}: db {bytes}")?;
    }

    writeln!(file, ";;;")?;
    writeln!(file, "segment .bss")?;
    writeln!(file, "        MEMORY: resb {}", MEMORY_SPACE)?;
    writeln!(file, ";;;")?;

    writeln!(file, "segment .text")?;
    subprogram::write_print_num(&mut file)?;
    subprogram::write_print_char(&mut file)?;

    writeln!(file, " ")?;

    Ok(0)
}

fn generate_operation(
    file: &mut std::fs::File,
    op: &Operation,
    strings: &mut Vec<String>
) -> Result<i32, io::Error> {
    writeln!(file, "    address_{}:", op.address)?;
    match &op.kind {
        /* --------------------------------- // Push -------------------------------- */
        OperationKind::PushInt32(value) => {
            writeln!(file, "    ;;push int32")?;
            writeln!(file, "        push {}", value)?;
        }
        OperationKind::PushInt64(value) => {
            writeln!(file, "    ;;push int64")?;
            writeln!(file, "        mov rax, {}", value)?;
            writeln!(file, "        push rax")?;
        }
        OperationKind::PushChar(value) => {
            writeln!(file, "    ;;push char")?;
            writeln!(file, "        push 0x{:02X}", *value as u32)?;
        }
        OperationKind::PushStr(value) => {
            let size = value.len();
            writeln!(file, "    ;;push str")?;
            writeln!(file, "        push {}", size)?;
            writeln!(file, "        push STRING_{}", strings.len())?;
            strings.push(value.clone());
        }
        /* ---------------------------------- // IO --------------------------------- */
        OperationKind::PrintInt64 => {
            writeln!(file, "    ;;print")?;
            writeln!(file, "        pop rdi")?;
            writeln!(file, "        call {}", subprogram::SUBPROGRAM_IDENTIFIER_PRINT_INTEGER_64BIT)?;
        }
        OperationKind::PrintChar => {
            writeln!(file, "    ;;printc")?;
            writeln!(file, "        pop  rdi")?;
            writeln!(file, "        call {}", subprogram::SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?; // either call a subprogram or inline it will see whats better
        }
        /* ------------------------------ // Intrinsic ------------------------------ */
        OperationKind::Intrinsic(intrinsic_type) => {
            match intrinsic_type {
                /* -------------------------------- // Stack --------------------------------- */
                IntrinsicType::Drop => {
                    writeln!(file, "    ;;drop")?;
                    writeln!(file, "        pop rax")?;
                }
                IntrinsicType::Duplicate => {
                    writeln!(file, "    ;;dup")?;
                    writeln!(file, "        pop  rax")?;
                    writeln!(file, "        push rax")?;
                    writeln!(file, "        push rax")?;
                }
                IntrinsicType::Over => {
                    writeln!(file, "    ;;over")?;
                    writeln!(file, "        pop  rax")?;
                    writeln!(file, "        pop  rbx")?;
                    writeln!(file, "        push rbx")?;
                    writeln!(file, "        push rax")?;
                    writeln!(file, "        push rbx")?;
                }
                IntrinsicType::Swap => {
                    writeln!(file, "    ;;swap")?;
                    writeln!(file, "        pop  rax")?;
                    writeln!(file, "        pop  rbx")?;
                    writeln!(file, "        push rax")?;
                    writeln!(file, "        push rbx")?;
                }
                IntrinsicType::Rotate => {
                    writeln!(file, "    ;;rotate")?;
                    writeln!(file, "        pop  rax")?;
                    writeln!(file, "        pop  rbx")?;
                    writeln!(file, "        pop  rcx")?;
                    writeln!(file, "        push rbx")?;
                    writeln!(file, "        push rcx")?;
                    writeln!(file, "        push rax")?;
                }
                /* ------------------------------ // Arithmetic ----------------------------- */
                IntrinsicType::Add => {
                    writeln!(file, "    ;;plus")?;
                    writeln!(file, "        pop  rax")?;
                    writeln!(file, "        pop  rbx")?;
                    writeln!(file, "        add  rbx, rax")?;
                    writeln!(file, "        push rbx")?;
                }
                IntrinsicType::Subtract => {
                    writeln!(file, "    ;;minus")?;
                    writeln!(file, "        pop  rax")?;
                    writeln!(file, "        pop  rbx")?;
                    writeln!(file, "        sub  rbx, rax")?;
                    writeln!(file, "        push rbx")?;
                }
                IntrinsicType::Multiply => {
                    writeln!(file, "    ;;mult")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        cqo")?;
                    writeln!(file, "        imul  rbx")?; // rax * rbx = rdx:rax (128-bit integer)
                    writeln!(file, "        push  rax")?;
                }
                IntrinsicType::DivMod => {
                    writeln!(file, "    ;;divide")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        cqo")?;
                    writeln!(file, "        idiv  rbx")?; // rax / rbx = rax     remainder rdx
                    writeln!(file, "        push  rax")?;
                    writeln!(file, "        push  rdx")?;
                }
                /* -------------------------------- // Logic -------------------------------- */
                IntrinsicType::Equal => {
                    writeln!(file, "    ;;eq")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        cmp   rbx, rax")?;
                    writeln!(file, "        mov   rbx, 0")?;
                    writeln!(file, "        mov   rax, 1")?;
                    writeln!(file, "        cmove rbx, rax")?;
                    writeln!(file, "        push  rbx")?;
                }
                IntrinsicType::NotEqual => {
                    writeln!(file, "    ;;not eq")?;
                    writeln!(file, "        pop    rax")?;
                    writeln!(file, "        pop    rbx")?;
                    writeln!(file, "        cmp    rbx, rax")?;
                    writeln!(file, "        mov    rbx, 0")?;
                    writeln!(file, "        mov    rax, 1")?;
                    writeln!(file, "        cmovne rbx, rax")?;
                    writeln!(file, "        push   rbx")?;
                }
                IntrinsicType::Less => {
                    writeln!(file, "    ;;le")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        cmp   rbx, rax")?;
                    writeln!(file, "        mov   rbx, 0")?;
                    writeln!(file, "        mov   rax, 1")?;
                    writeln!(file, "        cmovl rbx, rax")?;
                    writeln!(file, "        push  rbx")?;
                }
                IntrinsicType::Greater => {
                    writeln!(file, "    ;;gr")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        cmp   rbx, rax")?;
                    writeln!(file, "        mov   rbx, 0")?;
                    writeln!(file, "        mov   rax, 1")?;
                    writeln!(file, "        cmovg rbx, rax")?;
                    writeln!(file, "        push  rbx")?;
                }
                IntrinsicType::GreaterEqual => {
                    writeln!(file, "    ;;greq")?;
                    writeln!(file, "        pop    rax")?;
                    writeln!(file, "        pop    rbx")?;
                    writeln!(file, "        cmp    rbx, rax")?;
                    writeln!(file, "        mov    rbx, 0")?;
                    writeln!(file, "        mov    rax, 1")?;
                    writeln!(file, "        cmovge rbx, rax")?;
                    writeln!(file, "        push   rbx")?;
                }
                IntrinsicType::LessEqual => {
                    writeln!(file, "    ;;leeq")?;
                    writeln!(file, "        pop    rax")?;
                    writeln!(file, "        pop    rbx")?;
                    writeln!(file, "        cmp    rbx, rax")?;
                    writeln!(file, "        mov    rbx, 0")?;
                    writeln!(file, "        mov    rax, 1")?;
                    writeln!(file, "        cmovle rbx, rax")?;
                    writeln!(file, "        push   rbx")?;
                }
                IntrinsicType::Not => {
                    writeln!(file, "    ;;not")?;
                    writeln!(file, "        pop    rax")?;
                    writeln!(file, "        cmp    rax, 0")?;
                    writeln!(file, "        mov    rbx, 0")?;
                    writeln!(file, "        mov    rax, 1")?;
                    writeln!(file, "        cmovz  rbx, rax")?;
                    writeln!(file, "        push   rbx")?;
                }
                /* ------------------------------- // Bitwise ------------------------------- */
                IntrinsicType::BitNegate => {
                    writeln!(file, "    ;;bit neg")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        not   rax")?;
                    writeln!(file, "        push  rax")?;    
                }
                IntrinsicType::BitAnd => {
                    writeln!(file, "    ;;bit and")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        and   rbx, rax")?;
                    writeln!(file, "        push  rbx")?;
                }
                IntrinsicType::BitOr => {
                    writeln!(file, "    ;;bit or")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        or    rbx, rax")?;
                    writeln!(file, "        push  rbx")?;
                }
                IntrinsicType::ShiftRight => {
                    writeln!(file, "    ;;shift right")?;
                    writeln!(file, "        pop   rcx")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        shr   rbx, cl")?;
                    writeln!(file, "        push  rbx")?;
                }
                IntrinsicType::ShiftLeft => {
                    writeln!(file, "    ;;shift left")?;
                    writeln!(file, "        pop   rcx")?;
                    writeln!(file, "        pop   rbx")?;
                    writeln!(file, "        shl   rbx, cl")?;
                    writeln!(file, "        push  rbx")?;
                }
                /* -------------------------------- // Memory ------------------------------- */
                IntrinsicType::MemoryPush => {
                    writeln!(file, "    ;;mem")?;
                    writeln!(file, "        push MEMORY")?; 
                    writeln!(file, "        pop  rax")?;
                    writeln!(file, "        push rax")?;
                }
                IntrinsicType::MemoryLoad => {
                    writeln!(file, "    ;;load")?;
                    writeln!(file, "        pop   rax")?;
                    writeln!(file, "        xor   rbx, rbx")?;
                    writeln!(file, "        mov   bl, byte [rax]")?;
                    writeln!(file, "        push  rbx")?;
                }
                IntrinsicType::MemoryStore => {
                    writeln!(file, "    ;;store")?;
                    writeln!(file, "        pop rbx")?; // value
                    writeln!(file, "        pop rax")?; // address
                    writeln!(file, "        mov byte [rax], bl")?; // address
                }
                /* ------------------------------- // Syscall ------------------------------- */
                IntrinsicType::Syscall1 => {
                    writeln!(file, "    ;;syscall1")?;
                    writeln!(file, "        pop rax")?;
                    writeln!(file, "        pop rdi")?;
                    writeln!(file, "        syscall")?;
                }
                IntrinsicType::Syscall2 => {
                    writeln!(file, "    ;;syscall3")?;
                    writeln!(file, "        pop rax")?;
                    writeln!(file, "        pop rdi")?;
                    writeln!(file, "        pop rsi")?;
                    writeln!(file, "        syscall")?;
                }
                IntrinsicType::Syscall3 => {
                    writeln!(file, "    ;;syscall3")?;
                    writeln!(file, "        pop rax")?;
                    writeln!(file, "        pop rdi")?;
                    writeln!(file, "        pop rsi")?;
                    writeln!(file, "        pop rdx")?;
                    writeln!(file, "        syscall")?;
                }
                IntrinsicType::Syscall4 => {
                    writeln!(file, "    ;;syscall4")?;
                    writeln!(file, "        pop rax")?;
                    writeln!(file, "        pop rdi")?;
                    writeln!(file, "        pop rsi")?;
                    writeln!(file, "        pop rdx")?;
                    writeln!(file, "        pop r10")?;
                    writeln!(file, "        syscall")?;
                }
                IntrinsicType::Syscall5 => {
                    writeln!(file, "    ;;syscall5")?;
                    writeln!(file, "        pop rax")?;
                    writeln!(file, "        pop rdi")?;
                    writeln!(file, "        pop rsi")?;
                    writeln!(file, "        pop rdx")?;
                    writeln!(file, "        pop r10")?;
                    writeln!(file, "        pop r8")?;
                    writeln!(file, "        syscall")?;
                }
                IntrinsicType::Syscall6 => {
                    writeln!(file, "    ;;syscall6")?;
                    writeln!(file, "        pop rax")?;
                    writeln!(file, "        pop rdi")?;
                    writeln!(file, "        pop rsi")?;
                    writeln!(file, "        pop rdx")?;
                    writeln!(file, "        pop r10")?;
                    writeln!(file, "        pop r8")?;
                    writeln!(file, "        pop r9")?;
                    writeln!(file, "        syscall")?;
                }
            }
        }
        /* ---------------------------------- // Keyword --------------------------------- */
        OperationKind::If(jump) => {
            writeln!(file, "    ;;if")?;
            writeln!(file, "        pop rax")?;
            writeln!(file, "        cmp rax, 0")?;
            writeln!(file, "        jz  address_{}", jump.unwrap())?;
        }
        OperationKind::End(jump) => {
            writeln!(file, "    ;;end")?;
            if let Some(jump) = jump {
                writeln!(file, "        jmp address_{}", jump)?;
            }
        }
        OperationKind::Else(jump) => {
            writeln!(file, "    ;;else")?;
            writeln!(file, "        jmp address_{}", jump.unwrap())?;
        }
        OperationKind::Do(jump) => {
            writeln!(file, "    ;;do")?;
            writeln!(file, "        pop  rax")?;
            writeln!(file, "        cmp  rax, 0")?;
            writeln!(file, "        jz   address_{}", jump.unwrap())?;
        }
        OperationKind::While => {
            writeln!(file, "    ;;while")?;
        }
    }
    Ok(0)
}
