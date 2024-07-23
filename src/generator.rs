mod subprogram;

use std::fs::File;
use std::io::{self, Write};
use crate::operation::{Operation, OperationKind};

const STRING_SPACE: usize = 1_024;
const MEMORY_SPACE: usize = 64_000;
static mut STRING_SPACE_COUNTER: usize = 0;

pub fn generate_linux_nasm_x86_64(program: &Vec<Operation>, output: &str) -> Result<i32, std::io::Error> {
    let mut file = File::create(output).unwrap_or_else(|e| panic!("[ERROR]: Assembly file {output:?} creation failed: {e}"));
    writeln!(file, "bits 64")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .text")?;
    writeln!(file, "    global _start")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "    _start:")?;
    
    let mut strings: Vec<String> = Vec::new();
    for op in program {
        generate_operation(&mut file, op, &mut strings)?;
    }
    
    // writeln!(file, "address_{}:", program.len())?;
    writeln!(file, "        ;;; return")?;
    writeln!(file, "        mov rax, 60")?;
    writeln!(file, "        mov rdi, 0")?;
    writeln!(file, "        syscall")?;
    writeln!(file, "    ;;; _start")?;
    writeln!(file, "section .data")?;

    for (index, string) in strings.iter().enumerate() {
        let b: Vec<_> = string.as_bytes().iter()
            .map(|x| format!("0x{x:02X}"))
            .collect();
        let bytes = format!("{:?}", b)
            .replace(|c| c == '[' || c == ']' || c == '"', "");
        writeln!(file, "    str_{index}: db {bytes}")?;
    }

    writeln!(file, "    ;;;")?;
    writeln!(file, "section .bss")?;
    writeln!(file, "    MEMORY: resb {}", MEMORY_SPACE + STRING_SPACE)?;
    writeln!(file, "    ;;;")?;

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
    write!(file, "\taddress_{}:\t", op.address)?;
    match &op.kind {
        /* --------------------------------- // Push -------------------------------- */
        OperationKind::PushInt(value) => {
            writeln!(file, ";; push int")?;
            writeln!(file, "\t    push {}", value)?;
        }
        OperationKind::PushChar(value) => {
            writeln!(file, ";; push char")?;
            writeln!(file, "\t    push 0x{:02X}", *value as u32)?;
        }
        OperationKind::PushStr(value) => {
            let size = value.len();
            writeln!(file, ";; push str")?;
            writeln!(file, "\t    push {}", size)?;
            writeln!(file, "\t    push str_{}", strings.len())?;
            strings.push(value.to_string());
            unsafe {
                STRING_SPACE_COUNTER += size;
                assert!(STRING_SPACE_COUNTER < STRING_SPACE, "[ERROR]: string space overflow");
            }
        }
        /* ---------------------------------- // IO --------------------------------- */
        OperationKind::Dump => {
            writeln!(file, ";; dump")?;
            writeln!(file, "\t    pop rdi")?;
            writeln!(file, "\t    call {}", subprogram::SUBPROGRAM_IDENTIFIER_PRINT_INTEGER)?;
        }
        OperationKind::PrintChar => {
            writeln!(file, ";; printc")?;
            writeln!(file, "\t    pop  rdi")?;
            writeln!(file, "\t    call {}", subprogram::SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?; // either call a subprogram or inline it will see whats better
            // writeln!(file, "\t    mov  rax, 1")?;
            // writeln!(file, "\t    mov  rdi, 1")?;
            // writeln!(file, "\t    pop  r8")?;
            // writeln!(file, "\t    or   r8, 0x0a00")?; // adding new line
            // writeln!(file, "\t    push r8")?;
            // writeln!(file, "\t    mov  rsi, rsp")?;
            // writeln!(file, "\t    mov  rdx, 2")?;
            // writeln!(file, "\t    syscall")?; 
        }
        /* -------------------------------- // Stack -------------------------------- */
        OperationKind::Drop => {
            writeln!(file, ";; drop")?;
            writeln!(file, "\t    pop rax")?;
        }
        OperationKind::Duplicate => {
            writeln!(file, ";; dup")?;
            writeln!(file, "\t    pop  rax")?;
            writeln!(file, "\t    push rax")?;
            writeln!(file, "\t    push rax")?;
        }
        OperationKind::Duplicate2 => {
            writeln!(file, ";; dup 2")?;
            writeln!(file, "\t    pop  rax")?;
            writeln!(file, "\t    pop  rbx")?;
            writeln!(file, "\t    push rbx")?;
            writeln!(file, "\t    push rax")?;
            writeln!(file, "\t    push rbx")?;
            writeln!(file, "\t    push rax")?;
        }
        OperationKind::Over => {
            writeln!(file, ";; over")?;
            writeln!(file, "\t    pop  rax")?;
            writeln!(file, "\t    pop  rbx")?;
            writeln!(file, "\t    push rbx")?;
            writeln!(file, "\t    push rax")?;
            writeln!(file, "\t    push rbx")?;
        }
        OperationKind::Swap => {
            writeln!(file, ";; swap")?;
            writeln!(file, "\t    pop  rax")?;
            writeln!(file, "\t    pop  rbx")?;
            writeln!(file, "\t    push rax")?;
            writeln!(file, "\t    push rbx")?;
        }
        /* ------------------------------ // Arithmetic ----------------------------- */
        OperationKind::Add => {
            writeln!(file, ";; plus")?;
            writeln!(file, "\t    pop  rax")?;
            writeln!(file, "\t    pop  rbx")?;
            writeln!(file, "\t    add  rbx, rax")?;
            writeln!(file, "\t    push rbx")?;
        }
        OperationKind::Subtract => {
            writeln!(file, ";; minus")?;
            writeln!(file, "\t    pop  rax")?;
            writeln!(file, "\t    pop  rbx")?;
            writeln!(file, "\t    sub  rbx, rax")?;
            writeln!(file, "\t    push rbx")?;
        }
        OperationKind::Multiply => {
            writeln!(file, ";; mult")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    cqo")?;
            writeln!(file, "\t    imul  rbx")?; // rax * rbx = rdx:rax (128-bit integer)
            writeln!(file, "\t    push  rax")?;
        }
        OperationKind::Divide => {
            writeln!(file, ";; divide")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    cqo")?;
            writeln!(file, "\t    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "\t    push  rax")?;
        }
        OperationKind::Modulo => {
            writeln!(file, ";; modulo")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    cqo")?;
            writeln!(file, "\t    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "\t    push  rdx")?;
        }
        /* -------------------------------- // Logic -------------------------------- */
        OperationKind::Equal => {
            writeln!(file, ";; eq")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    cmp   rbx, rax")?;
            writeln!(file, "\t    mov   rbx, 0")?;
            writeln!(file, "\t    mov   rax, 1")?;
            writeln!(file, "\t    cmove rbx, rax")?;
            writeln!(file, "\t    push  rbx")?;
        }
        OperationKind::NotEqual => {
            writeln!(file, ";; not eq")?;
            writeln!(file, "\t    pop    rax")?;
            writeln!(file, "\t    pop    rbx")?;
            writeln!(file, "\t    cmp    rbx, rax")?;
            writeln!(file, "\t    mov    rbx, 0")?;
            writeln!(file, "\t    mov    rax, 1")?;
            writeln!(file, "\t    cmovne rbx, rax")?;
            writeln!(file, "\t    push   rbx")?;
        }
        OperationKind::Less => {
            writeln!(file, ";; le")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    cmp   rbx, rax")?;
            writeln!(file, "\t    mov   rbx, 0")?;
            writeln!(file, "\t    mov   rax, 1")?;
            writeln!(file, "\t    cmovl rbx, rax")?;
            writeln!(file, "\t    push  rbx")?;
        }
        OperationKind::Greater => {
            writeln!(file, ";; gr")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    cmp   rbx, rax")?;
            writeln!(file, "\t    mov   rbx, 0")?;
            writeln!(file, "\t    mov   rax, 1")?;
            writeln!(file, "\t    cmovg rbx, rax")?;
            writeln!(file, "\t    push  rbx")?;
        }
        OperationKind::GreaterEqual => {
            writeln!(file, ";; eqgr")?;
            writeln!(file, "\t    pop    rax")?;
            writeln!(file, "\t    pop    rbx")?;
            writeln!(file, "\t    cmp    rbx, rax")?;
            writeln!(file, "\t    mov    rbx, 0")?;
            writeln!(file, "\t    mov    rax, 1")?;
            writeln!(file, "\t    cmovge rbx, rax")?;
            writeln!(file, "\t    push   rbx")?;
        }
        OperationKind::LessEqual => {
            writeln!(file, ";; eqle")?;
            writeln!(file, "\t    pop    rax")?;
            writeln!(file, "\t    pop    rbx")?;
            writeln!(file, "\t    cmp    rbx, rax")?;
            writeln!(file, "\t    mov    rbx, 0")?;
            writeln!(file, "\t    mov    rax, 1")?;
            writeln!(file, "\t    cmovle rbx, rax")?;
            writeln!(file, "\t    push   rbx")?;
        }
        OperationKind::Not => {
            writeln!(file, ";; not")?;
            writeln!(file, "\t    pop    rax")?;
            writeln!(file, "\t    cmp    rax, 0")?;
            writeln!(file, "\t    mov    rbx, 0")?;
            writeln!(file, "\t    mov    rax, 1")?;
            writeln!(file, "\t    cmovz  rbx, rax")?;
            writeln!(file, "\t    push   rbx")?;
        }
        /* ------------------------------- // Bitwise ------------------------------- */
        OperationKind::BitAnd => {
            writeln!(file, ";; bit and")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    and   rbx, rax")?;
            writeln!(file, "\t    push  rbx")?;
        }
        OperationKind::BitOr => {
            writeln!(file, ";; bit or")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    or    rbx, rax")?;
            writeln!(file, "\t    push  rbx")?;
        }
        OperationKind::ShiftRight => {
            writeln!(file, ";; shift right")?;
            writeln!(file, "\t    pop   rcx")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    shr   rbx, cl")?;
            writeln!(file, "\t    push  rbx")?;
        }
        OperationKind::ShiftLeft => {
            writeln!(file, ";; shift left")?;
            writeln!(file, "\t    pop   rcx")?;
            writeln!(file, "\t    pop   rbx")?;
            writeln!(file, "\t    shl   rbx, cl")?;
            writeln!(file, "\t    push  rbx")?;
        }
        /* ---------------------------------- // Block --------------------------------- */
        OperationKind::If(jump) => {
            writeln!(file, ";; if")?;
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    cmp rax, 0")?;
            writeln!(file, "\t    jz  address_{}", jump)?;
        }
        OperationKind::End(jump) => {
            writeln!(file, ";; end")?;
            if *jump >= 0 {
                writeln!(file, "\t    jmp address_{}", jump)?;
            }
        }
        OperationKind::Else(jump) => {
            writeln!(file, ";; else")?;
            writeln!(file, "\t    jmp address_{}", jump)?;
        }
        OperationKind::Do(jump) => {
            writeln!(file, ";; do")?;
            writeln!(file, "\t    pop  rax")?;
            writeln!(file, "\t    cmp  rax, 0")?;
            writeln!(file, "\t    jz   address_{}", jump)?;
        }
        OperationKind::While => {
            writeln!(file, ";; while")?;
            writeln!(file, "\t    ;  ignore")?;
        }
        /* -------------------------------- // Preprocessor -------------------------- */
        OperationKind::Macro => {
            unreachable!("macro should have been gone by now");
        }
        OperationKind::Include(file) => {
            panic!("include file not inplemeted: {file}");
        }
        /* -------------------------------- // Memory ------------------------------- */
        OperationKind::MemoryPush => {
            writeln!(file, ";; mem")?;
            // push the address of MEMORY in .bss
            writeln!(file, "\t    push MEMORY")?; 
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    add rax, {}", STRING_SPACE)?;
            writeln!(file, "\t    push rax")?;
        }
        OperationKind::MemoryLoad => {
            writeln!(file, ";; load")?;
            writeln!(file, "\t    pop   rax")?;
            writeln!(file, "\t    xor   rbx, rbx")?;
            writeln!(file, "\t    mov   bl, byte [rax]")?;
            writeln!(file, "\t    push  rbx")?;
        }
        OperationKind::MemoryStore => {
            writeln!(file, ";; store")?;
            writeln!(file, "\t    pop rbx")?; // value
            writeln!(file, "\t    pop rax")?; // address
            writeln!(file, "\t    mov byte [rax], bl")?; // address
        }
        /* ------------------------------- // Syscall ------------------------------- */
        OperationKind::Syscall1 => {
            writeln!(file, ";; syscall1")?;
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    pop rdi")?;
            writeln!(file, "\t    syscall")?;
        }
        OperationKind::Syscall2 => {
            writeln!(file, ";; syscall3")?;
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    pop rdi")?;
            writeln!(file, "\t    pop rsi")?;
            writeln!(file, "\t    syscall")?;
        }
        OperationKind::Syscall3 => {
            writeln!(file, ";; syscall3")?;
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    pop rdi")?;
            writeln!(file, "\t    pop rsi")?;
            writeln!(file, "\t    pop rdx")?;
            writeln!(file, "\t    syscall")?;
        }
        OperationKind::Syscall4 => {
            writeln!(file, ";; syscall4")?;
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    pop rdi")?;
            writeln!(file, "\t    pop rsi")?;
            writeln!(file, "\t    pop rdx")?;
            writeln!(file, "\t    pop r10")?;
            writeln!(file, "\t    syscall")?;
        }
        OperationKind::Syscall5 => {
            writeln!(file, ";; syscall5")?;
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    pop rdi")?;
            writeln!(file, "\t    pop rsi")?;
            writeln!(file, "\t    pop rdx")?;
            writeln!(file, "\t    pop r10")?;
            writeln!(file, "\t    pop r8")?;
            writeln!(file, "\t    syscall")?;
        }
        OperationKind::Syscall6 => {
            writeln!(file, ";; syscall6")?;
            writeln!(file, "\t    pop rax")?;
            writeln!(file, "\t    pop rdi")?;
            writeln!(file, "\t    pop rsi")?;
            writeln!(file, "\t    pop rdx")?;
            writeln!(file, "\t    pop r10")?;
            writeln!(file, "\t    pop r8")?;
            writeln!(file, "\t    pop r9")?;
            writeln!(file, "\t    syscall")?;
        }
    }
    Ok(0)
}
