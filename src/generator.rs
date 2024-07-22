use std::fs::File;
use std::io::{self, Write};
use crate::operation::{Operation, OperationKind};

const STRING_SPACE: usize = 1_024;
const MEMORY_SPACE: usize = 64_000;
static mut STRING_SPACE_COUNTER: usize = 0;

pub fn generate_linux_nasm_x86_64(program: &Vec<Operation>, output: &str) -> io::Result<i32> {
    let mut file = File::create(output).expect("creation failed");
    writeln!(file, "bits 64")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "_start:")?;
    
    let mut strings: Vec<String> = Vec::new();
    for (index, op) in program.iter().enumerate() {
        generate_operation(&mut file, index, op, &mut strings)?;
    }
    
    // writeln!(file, "address_{}:", program.len())?;
    writeln!(file, "    ;;; return")?;
    writeln!(file, "    mov rax, 60")?;
    writeln!(file, "    mov rdi, 0")?;
    writeln!(file, "    syscall")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .data")?;

    for (index, string) in strings.iter().enumerate() {
        let b: Vec<_> = string.as_bytes().iter()
            .map(|x| format!("0x{x:02x}"))
            .collect();
        let bytes = format!("{:?}", b)
            .replace(|c| c == '[' || c == ']' || c == '"', "");
        writeln!(file, "    str_{index}: db {bytes}")?;
    }

    writeln!(file, "    ;;;")?;
    writeln!(file, "section .bss")?;
    writeln!(file, "    MEMORY: resb {}", MEMORY_SPACE + STRING_SPACE)?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .text")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "    global _start")?;
    writeln!(file, "    global print_num")?;
    writeln!(file, "    global sys_write_stdout")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "sys_write_stdout:")?;
    writeln!(file, "    enter 0, 0")?;
    writeln!(file, "    mov   rax, 1")?;
    writeln!(file, "    mov   rdi, 1")?;
    writeln!(file, "    syscall")?;
    writeln!(file, "    leave")?;
    writeln!(file, "    ret")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "print_num:")?;
    writeln!(file, "    enter 0, 0")?;
    writeln!(file, "        cmp  rdi, 0")?;
    writeln!(file, "        mov  rcx, 1")?;
    writeln!(file, "        je   .done1")?;
    writeln!(file, "        jge  .positive")?;
    writeln!(file, "        neg  rdi")?;
    writeln!(file, "        push rdi")?;
    writeln!(file, "        push '-'")?;
    writeln!(file, "        mov  rsi, rsp")?;
    writeln!(file, "        mov  rdx, 1")?;
    writeln!(file, "        call sys_write_stdout")?;
    writeln!(file, "        pop  rdi")?; // dropping '-'
    writeln!(file, "        pop  rdi")?;
    writeln!(file, "    .positive:")?;
    writeln!(file, "        mov rax, rdi")?;
    writeln!(file, "        mov rbx, 10")?;
    writeln!(file, "        xor rcx, rcx")?;
    writeln!(file, "    .loop1:")?;
    writeln!(file, "        cmp rax, 0")?;
    writeln!(file, "        jle .done1")?;
    writeln!(file, "        cdq")?;
    writeln!(file, "        idiv rbx")?;
    writeln!(file, "        inc rcx")?;
    writeln!(file, "        jmp .loop1")?;
    writeln!(file, "    .done1:")?;
    writeln!(file, "        inc rcx")?;
    writeln!(file, "        mov r8, rcx")?;
    writeln!(file, "        sub rsp, rcx")?;
    writeln!(file, "        dec rcx")?;
    writeln!(file, "        mov byte [rsp + rcx], 0x0A")?;
    writeln!(file, "        dec rcx")?;
    writeln!(file, "        cmp rdi, 0")?;
    writeln!(file, "        jnz .skip")?;
    writeln!(file, "        mov byte [rsp + rcx], '0'")?;
    writeln!(file, "        jmp .done2")?;
    writeln!(file, "    .skip:")?;
    writeln!(file, "        mov rax, rdi")?;
    writeln!(file, "    .loop2:")?;
    writeln!(file, "        cmp rcx, 0")?;
    writeln!(file, "        jl  .done2")?;
    writeln!(file, "        cdq")?;
    writeln!(file, "        idiv rbx")?;
    writeln!(file, "        add  rdx, '0'")?;
    writeln!(file, "        mov  byte [rsp + rcx], dl")?;
    writeln!(file, "        dec  rcx")?;
    writeln!(file, "        jmp  .loop2")?;
    writeln!(file, "    .done2:")?;
    writeln!(file, "        mov  rsi, rsp")?;
    writeln!(file, "        mov  rdx, r8")?;
    writeln!(file, "        call sys_write_stdout")?;
    writeln!(file, "    leave")?;
    writeln!(file, "    ret")?;
    writeln!(file, " ")?;

    Ok(0)
}

fn generate_operation(
    file: &mut std::fs::File,
    index: usize,
    op: &Operation,
    strings: &mut Vec<String>
) -> Result<i32, io::Error> {
    writeln!(file, "address_{}:", op.address)?;
    match &op.kind {
        /* --------------------------------- // Push -------------------------------- */
        OperationKind::PushInt(value) => {
            writeln!(file, "    ;; push op")?;
            writeln!(file, "    push {}", value)?;
        }
        OperationKind::PushStr(value) => {
            let size = value.len();
            // writeln!(file, "    ;; push str")?;
            // writeln!(file, "    push {}", size)?;                           // pushing the string length
            // writeln!(file, "    lea rax, [rel MEMORY]")?;                   // loading memory address to register
            // unsafe {
            //     writeln!(file, "    add rax, {}", STRING_SPACE_COUNTER)?;   // offsetting
            // }
            // writeln!(file, "    push rax")?;                                // push the crafted address
            // for i in 0..size {
            //     let char = value.as_bytes()[i];
            //     writeln!(file, "    mov  byte [rax + {i}], {char}")?;      
            // }
            writeln!(file, "    ;; push str")?;
            writeln!(file, "    push {}", size)?;
            writeln!(file, "    push str_{}", strings.len())?;
            strings.push(value.to_string());
            unsafe {
                STRING_SPACE_COUNTER += size;
                assert!(STRING_SPACE_COUNTER < STRING_SPACE, "[ERROR]: string space overflow");
            }
        }
        /* -------------------------------- // Stack -------------------------------- */
        OperationKind::Dump => {
            writeln!(file, "    ;; dump")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    call print_num")?;
        }
        OperationKind::Drop => {
            writeln!(file, "    ;; drop")?;
            writeln!(file, "    pop rax")?;
        }
        OperationKind::Duplicate => {
            writeln!(file, "    ;; dup")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rax")?;
        }
        OperationKind::Duplicate2 => {
            writeln!(file, "    ;; dup 2")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
        }
        OperationKind::Over => {
            writeln!(file, "    ;; over")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
        }
        OperationKind::Swap => {
            writeln!(file, "    ;; swap")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
        }
        /* ------------------------------ // Arithmetic ----------------------------- */
        OperationKind::Add => {
            writeln!(file, "    ;; plus")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    add  rbx, rax")?;
            writeln!(file, "    push rbx")?;
        }
        OperationKind::Subtract => {
            writeln!(file, "    ;; minus")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    sub  rbx, rax")?;
            writeln!(file, "    push rbx")?;
        }
        OperationKind::Multiply => {
            writeln!(file, "    ;; mult")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    imul  rbx")?; // rax * rbx = rdx:rax (128-bit integer)
            writeln!(file, "    push  rax")?;
        }
        OperationKind::Divide => {
            writeln!(file, "    ;; divide")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "    push  rax")?;
        }
        OperationKind::Modulo => {
            writeln!(file, "    ;; modulo")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "    push  rdx")?;
        }
        /* -------------------------------- // Logic -------------------------------- */
        OperationKind::Equal => {
            writeln!(file, "    ;; eq")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmove rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        OperationKind::NotEqual => {
            writeln!(file, "    ;; not eq")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovne rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        OperationKind::Less => {
            writeln!(file, "    ;; le")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmovl rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        OperationKind::Greater => {
            writeln!(file, "    ;; gr")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmovg rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        OperationKind::GreaterEqual => {
            writeln!(file, "    ;; eqgr")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovge rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        OperationKind::LessEqual => {
            writeln!(file, "    ;; eqle")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovle rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        OperationKind::Not => {
            writeln!(file, "    ;; not")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    cmp    rax, 0")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovz  rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        /* ------------------------------- // Bitwise ------------------------------- */
        OperationKind::BitAnd => {
            writeln!(file, "    ;; bit and")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    and   rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        OperationKind::BitOr => {
            writeln!(file, "    ;; bit or")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    or    rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        OperationKind::ShiftRight => {
            writeln!(file, "    ;; shift right")?;
            writeln!(file, "    pop   rcx")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    shr   rbx, cl")?;
            writeln!(file, "    push  rbx")?;
        }
        OperationKind::ShiftLeft => {
            writeln!(file, "    ;; shift left")?;
            writeln!(file, "    pop   rcx")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    shl   rbx, cl")?;
            writeln!(file, "    push  rbx")?;
        }
        /* ---------------------------------- // Block --------------------------------- */
        OperationKind::If(jump) => {
            writeln!(file, "    ;; if")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    cmp rax, 0")?;
            writeln!(file, "    jz  address_{}", jump)?;
        }
        OperationKind::Else(jump) => {
            writeln!(file, "    ;; else")?;
            writeln!(file, "    jmp address_{}", jump)?;
        }
        OperationKind::Do(jump) => {
            writeln!(file, "    ;; do")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    cmp  rax, 0")?;
            writeln!(file, "    jz   address_{}", jump)?;
        }
        OperationKind::While => {
            writeln!(file, "    ;; while")?;
            writeln!(file, "    ;  ignore")?;
        }
        OperationKind::Macro => {
            unreachable!("macro should have been gone by now");
        }
        OperationKind::End(jump) => {
            writeln!(file, "    ;; end")?;
            if *jump >= 0 {
                writeln!(file, "    jmp address_{}", jump)?;
            }
        }
        /* -------------------------------- // Memory ------------------------------- */
        OperationKind::MemoryPush => {
            writeln!(file, "    ;; mem")?;
            // push the address of MEMORY in .bss
            writeln!(file, "    push MEMORY")?; 
            writeln!(file, "    pop rax")?;
            writeln!(file, "    add rax, {}", STRING_SPACE)?;
            writeln!(file, "    push rax")?;
        }
        OperationKind::MemoryLoad => {
            writeln!(file, "    ;; load")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    xor   rbx, rbx")?;
            writeln!(file, "    mov   bl, byte [rax]")?;
            writeln!(file, "    push  rbx")?;
        }
        OperationKind::MemoryStore => {
            writeln!(file, "    ;; store")?;
            writeln!(file, "    pop rbx")?; // value
            writeln!(file, "    pop rax")?; // address
            writeln!(file, "    mov byte [rax], bl")?; // address
        }
        /* ------------------------------- // Syscall ------------------------------- */
        OperationKind::Syscall1 => {
            writeln!(file, "    ;; syscall1")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    syscall")?;
        }
        OperationKind::Syscall2 => {
            writeln!(file, "    ;; syscall3")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    pop rsi")?;
            writeln!(file, "    syscall")?;
        }
        OperationKind::Syscall3 => {
            writeln!(file, "    ;; syscall3")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    pop rsi")?;
            writeln!(file, "    pop rdx")?;
            writeln!(file, "    syscall")?;
        }
        OperationKind::Syscall4 => {
            writeln!(file, "    ;; syscall4")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    pop rsi")?;
            writeln!(file, "    pop rdx")?;
            writeln!(file, "    pop r10")?;
            writeln!(file, "    syscall")?;
        }
        OperationKind::Syscall5 => {
            writeln!(file, "    ;; syscall5")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    pop rsi")?;
            writeln!(file, "    pop rdx")?;
            writeln!(file, "    pop r10")?;
            writeln!(file, "    pop r8")?;
            writeln!(file, "    syscall")?;
        }
        OperationKind::Syscall6 => {
            writeln!(file, "    ;; syscall6")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    pop rsi")?;
            writeln!(file, "    pop rdx")?;
            writeln!(file, "    pop r10")?;
            writeln!(file, "    pop r8")?;
            writeln!(file, "    pop r9")?;
            writeln!(file, "    syscall")?;
        }
    }
    Ok(0)
}
