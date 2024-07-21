use core::panic;
use std::fs::File;
use std::io::{self, Write};
use std::mem;
use crate::operation::{Operation, OperationType, OperationValue};

const MEMORY_SIZE: usize = 64_000;

pub fn create_assembly(program: &Vec<Operation>, output: &str) -> io::Result<i32> {
    let mut file = File::create(output).expect("creation failed");
    writeln!(file, "bits 64")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .data")?;
    writeln!(file, "    MINUS:          db '-', 0x00")?;
    writeln!(file, "    FD_STDOUT:      dq 1")?;
    writeln!(file, "    SYSCALL_WRITE:  dq 1")?;
    writeln!(file, "    SYSCALL_EXIT:   dq 60")?;
    writeln!(
        file,
        "    ALPHABET_UPPER: db \"abcdefghijklmnopqrstuvwxyz\", 0x00"
    )?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .bss")?;
    writeln!(file, "    MEMORY: resb {}", MEMORY_SIZE)?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .text")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "    global _start")?;
    writeln!(file, "    global print_num")?;
    writeln!(file, "    global sys_write_stdout")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "sys_write_stdout:")?;
    writeln!(file, "    enter 0, 0")?;
    writeln!(file, "    mov   rax, [SYSCALL_WRITE]")?;
    writeln!(file, "    mov   rdi, [FD_STDOUT]")?;
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
    writeln!(file, "        mov  rsi, MINUS")?;
    writeln!(file, "        mov  rdx, 1")?;
    writeln!(file, "        call sys_write_stdout")?;
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
    writeln!(file, "_start:")?;

    for (index, op) in program.iter().enumerate() {
        generate_operation(&mut file, index, op)?;
    }

    // writeln!(file, "address_{}:", program.len())?;
    writeln!(file, "    ;;; return")?;
    writeln!(file, "    mov rax, [SYSCALL_EXIT]")?;
    writeln!(file, "    mov rdi, 0")?;
    writeln!(file, "    syscall")?;

    Ok(0)
}

fn generate_operation(
    file: &mut std::fs::File,
    index: usize,
    op: &Operation,
) -> Result<i32, io::Error> {
    writeln!(file, "address_{}:", index)?;
    match (&op.op_type, &op.value) {
        /* --------------------------------- // Push -------------------------------- */
        (OperationType::PushInt, OperationValue::Int(value)) => {
            writeln!(file, "    ;; push op")?;
            writeln!(file, "    push {}", value)?;
        }
        (OperationType::PushStr, OperationValue::Str(_value)) => {
            todo!();
        }
        /* -------------------------------- // Stack -------------------------------- */
        (OperationType::Dump, OperationValue::Nothing) => {
            writeln!(file, "    ;; dump")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    call print_num")?;
        }
        (OperationType::Drop, OperationValue::Nothing) => {
            writeln!(file, "    ;; drop")?;
            writeln!(file, "    add rsp, {}", mem::size_of::<usize>())?; // system dependent
        }
        (OperationType::Duplicate, OperationValue::Nothing) => {
            writeln!(file, "    ;; dup")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rax")?;
        }
        (OperationType::Duplicate2, OperationValue::Nothing) => {
            writeln!(file, "    ;; dup 2")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
        }
        (OperationType::Over, OperationValue::Nothing) => {
            writeln!(file, "    ;; over")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
        }
        (OperationType::Swap, OperationValue::Nothing) => {
            writeln!(file, "    ;; swap")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
        }
        /* ------------------------------ // Arithmetic ----------------------------- */
        (OperationType::Add, OperationValue::Nothing) => {
            writeln!(file, "    ;; plus")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    add  rbx, rax")?;
            writeln!(file, "    push rbx")?;
        }
        (OperationType::Subtract, OperationValue::Nothing) => {
            writeln!(file, "    ;; minus")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    sub  rbx, rax")?;
            writeln!(file, "    push rbx")?;
        }
        (OperationType::Multiply, OperationValue::Nothing) => {
            writeln!(file, "    ;; mult")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    imul  rbx")?; // rax * rbx = rdx:rax (128-bit integer)
            writeln!(file, "    push  rax")?;
        }
        (OperationType::Divide, OperationValue::Nothing) => {
            writeln!(file, "    ;; divide")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "    push  rax")?;
        }
        (OperationType::Modulo, OperationValue::Nothing) => {
            writeln!(file, "    ;; modulo")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "    push  rdx")?;
        }
        /* -------------------------------- // Logic -------------------------------- */
        (OperationType::Equal, OperationValue::Nothing) => {
            writeln!(file, "    ;; eq")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmove rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        (OperationType::NotEqual, OperationValue::Nothing) => {
            writeln!(file, "    ;; not eq")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovne rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        (OperationType::Less, OperationValue::Nothing) => {
            writeln!(file, "    ;; le")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmovl rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        (OperationType::Greater, OperationValue::Nothing) => {
            writeln!(file, "    ;; gr")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmovg rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        (OperationType::GreaterEqual, OperationValue::Nothing) => {
            writeln!(file, "    ;; eqgr")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovge rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        (OperationType::LessEqual, OperationValue::Nothing) => {
            writeln!(file, "    ;; eqle")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovle rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        (OperationType::Not, OperationValue::Nothing) => {
            writeln!(file, "    ;; not")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    cmp    rax, 0")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovz  rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        /* ------------------------------- // Bitwise ------------------------------- */
        (OperationType::BitAnd, OperationValue::Nothing) => {
            writeln!(file, "    ;; bit and")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    and   rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        (OperationType::BitOr, OperationValue::Nothing) => {
            writeln!(file, "    ;; bit or")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    or    rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        (OperationType::ShiftRight, OperationValue::Nothing) => {
            writeln!(file, "    ;; shift right")?;
            writeln!(file, "    pop   rcx")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    shr   rbx, cl")?;
            writeln!(file, "    push  rbx")?;
        }
        (OperationType::ShiftLeft, OperationValue::Nothing) => {
            writeln!(file, "    ;; shift left")?;
            writeln!(file, "    pop   rcx")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    shl   rbx, cl")?;
            writeln!(file, "    push  rbx")?;
        }
        /* ---------------------------------- // Block --------------------------------- */
        (OperationType::End, OperationValue::Nothing) => {
            writeln!(file, "    ;; end")?;
            if op.jump >= 0 {
                writeln!(file, "    jmp address_{}", op.jump)?;
            }
        }
        (OperationType::If, OperationValue::Nothing) => {
            writeln!(file, "    ;; if")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    cmp rax, 0")?;
            writeln!(file, "    jz  address_{}", op.jump)?;
        }
        (OperationType::Else, OperationValue::Nothing) => {
            writeln!(file, "    ;; else")?;
            writeln!(file, "    jmp address_{}", op.jump)?;
        }
        (OperationType::Do, OperationValue::Nothing) => {
            writeln!(file, "    ;; do")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    cmp  rax, 0")?;
            writeln!(file, "    jz   address_{}", op.jump)?;
        }
        (OperationType::While, OperationValue::Nothing) => {
            writeln!(file, "    ;; while")?;
            writeln!(file, "    ;  ignore")?;
        }
        /* -------------------------------- // Memory ------------------------------- */
        (OperationType::MemoryPush, OperationValue::Nothing) => {
            writeln!(file, "    ;; mem")?;
            writeln!(file, "    push MEMORY")?; // push the address of MEMORY in .bss
        }
        (OperationType::MemoryLoad, OperationValue::Nothing) => {
            writeln!(file, "    ;; load")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    xor   rbx, rbx")?;
            writeln!(file, "    mov   bl, byte [rax]")?;
            writeln!(file, "    push  rbx")?;
        }
        (OperationType::MemoryStore, OperationValue::Nothing) => {
            writeln!(file, "    ;; store")?;
            writeln!(file, "    pop rbx")?; // value
            writeln!(file, "    pop rax")?; // address
            writeln!(file, "    mov byte [rax], bl")?; // address
        }
        /* ------------------------------- // Syscall ------------------------------- */
        (OperationType::Syscall1, OperationValue::Nothing) => {
            writeln!(file, "    ;; syscall1")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    syscall")?;
        }
        (OperationType::Syscall3, OperationValue::Nothing) => {
            writeln!(file, "    ;; syscall3")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    pop rsi")?;
            writeln!(file, "    pop rdx")?;
            writeln!(file, "    syscall")?;
        }
        (op_type, op_value) => panic!("Unexpected OperationType and OperationValue combination: type: {:?}, value: {:?}", op_type, op_value)
    }
    Ok(0)
}
