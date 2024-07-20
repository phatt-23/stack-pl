use std::mem;
use std::io::{self, Write};
use std::fs::File;
use std::process::Command;

use super::operation::{Operation, Token};

fn print_command_output(output: std::process::Output) {
    if !&output.stdout.is_empty() {
        println!("[INFO]: stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
    if !&output.stderr.is_empty() {
        println!("[ERROR]: stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        println!("[ERROR]: status:\n{}", output.status);
    }
}

pub fn compile_assembly(executable: &str) {
    print_command_output( Command::new("nasm").arg("-felf64").arg("output.asm").output().expect("nasm failed") );
    print_command_output( Command::new("ld").arg("output.o").arg("-o").arg(executable).output().expect("ld failed") );
}

pub fn create_assembly(program: &Vec<Operation>, output: &str) -> io::Result<i32> {
    let mut file = File::create(output).expect("creation failed");
    writeln!(file, "bits 64")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .data")?;
    writeln!(file, "    MINUS:          db '-', 0x00")?;
    writeln!(file, "    FD_STDOUT:      dq 1")?;
    writeln!(file, "    SYSCALL_WRITE:  dq 1")?;
    writeln!(file, "    SYSCALL_EXIT:   dq 60")?;
    writeln!(file, "    ALPHABET_UPPER: db \"abcdefghijklmnopqrstuvwxyz\", 0x00")?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "section .bss")?;
    writeln!(file, "    MEMORY: resb {}", 1_000_000)?;
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

    for op in program.into_iter() {
        parse_word_to_op(&mut file, op)?;
    }

    // writeln!(file, "address_{}:", program.len())?;
    writeln!(file, "    ;;; return")?;
    writeln!(file, "    mov rax, [SYSCALL_EXIT]")?;
    writeln!(file, "    mov rdi, 0")?;
    writeln!(file, "    syscall")?;

    Ok(0)
}

fn parse_word_to_op(file: &mut std::fs::File, op: &Operation) -> Result<i32, io::Error> {
    writeln!(file, "address_{}:", op.index)?;
    match op.token {
        Token::Push => {
            writeln!(file, "    ;; push op")?;
            writeln!(file, "    push {}", op.value)?;
        }
        Token::Dump => {
            writeln!(file, "    ;; dump")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    call print_num")?;
        }
        Token::Drop => {
            writeln!(file, "    ;; drop")?;
            writeln!(file, "    add rsp, {}", mem::size_of::<usize>())?; // system dependent
        }
        Token::Dup2 => {
            writeln!(file, "    ;; dup 2")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
        }            
        Token::Dup => {
            writeln!(file, "    ;; dup")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rax")?;
        }
        Token::Over => {
            writeln!(file, "    ;; over")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
        }
        Token::Swap => {
            writeln!(file, "    ;; swap")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    push rax")?;
            writeln!(file, "    push rbx")?;
        }
        Token::Plus => {
            writeln!(file, "    ;; plus")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    add  rbx, rax")?;
            writeln!(file, "    push rbx")?;
        }
        Token::Minus => {
            writeln!(file, "    ;; minus")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    pop  rbx")?;
            writeln!(file, "    sub  rbx, rax")?;
            writeln!(file, "    push rbx")?;
        }
        Token::Eq => {
            writeln!(file, "    ;; eq")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmove rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::NotEq => {
            writeln!(file, "    ;; not eq")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovne rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        Token::Le => {
            writeln!(file, "    ;; le")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmovl rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::Gr => {
            writeln!(file, "    ;; gr")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    cmp   rbx, rax")?;
            writeln!(file, "    mov   rbx, 0")?;
            writeln!(file, "    mov   rax, 1")?;
            writeln!(file, "    cmovg rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::BitAnd => {
            writeln!(file, "    ;; bit and")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    and   rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::BitOr => {
            writeln!(file, "    ;; bit or")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    or    rbx, rax")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::ShiftRight => {
            writeln!(file, "    ;; shift right")?;
            writeln!(file, "    pop   rcx")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    shr   rbx, cl")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::ShiftLeft => {
            writeln!(file, "    ;; shift left")?;
            writeln!(file, "    pop   rcx")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    shl   rbx, cl")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::End => {
            writeln!(file, "    ;; end")?;
            if op.value >= 0 {
                writeln!(file, "    jmp address_{}", op.value)?;
            }
        }
        Token::If => {
            writeln!(file, "    ;; if")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    cmp rax, 0")?;
            writeln!(file, "    jz  address_{}", op.value)?;
        }
        Token::Else => {
            writeln!(file, "    ;; else")?;
            writeln!(file, "    jmp address_{}", op.value)?;
        }
        Token::Do => {
            writeln!(file, "    ;; do")?;
            writeln!(file, "    pop  rax")?;
            writeln!(file, "    cmp  rax, 0")?;
            writeln!(file, "    jz   address_{}", op.value)?;
        }
        Token::While => {
            writeln!(file, "    ;; while")?;
            writeln!(file, "    ;  ignore")?;
        }
        Token::EqGr => {
            writeln!(file, "    ;; eqgr")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovge rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        Token::EqLe => {
            writeln!(file, "    ;; eqle")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    pop    rbx")?;
            writeln!(file, "    cmp    rbx, rax")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovle rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        Token::Not => {
            writeln!(file, "    ;; not")?;
            writeln!(file, "    pop    rax")?;
            writeln!(file, "    cmp    rax, 0")?;
            writeln!(file, "    mov    rbx, 0")?;
            writeln!(file, "    mov    rax, 1")?;
            writeln!(file, "    cmovz  rbx, rax")?;
            writeln!(file, "    push   rbx")?;
        }
        Token::Multiply => {
            writeln!(file, "    ;; mult")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    imul  rbx")?; // rax * rbx = rdx:rax (128-bit integer)
            writeln!(file, "    push  rax")?;
        }
        Token::Divide => {
            writeln!(file, "    ;; divide")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "    push  rax")?;
        }
        Token::Modulo => {
            writeln!(file, "    ;; modulo")?;
            writeln!(file, "    pop   rbx")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    cqo")?;
            writeln!(file, "    idiv  rbx")?; // rax / rbx = rax     remainder rdx
            writeln!(file, "    push  rdx")?;
        }
        Token::Memory => {
            writeln!(file, "    ;; mem")?;
            writeln!(file, "    push MEMORY")?; // push the address of MEMORY in .bss
        }
        Token::Load => {
            writeln!(file, "    ;; load")?;
            writeln!(file, "    pop   rax")?;
            writeln!(file, "    xor   rbx, rbx")?;
            writeln!(file, "    mov   bl, byte [rax]")?;
            writeln!(file, "    push  rbx")?;
        }
        Token::Store => {
            writeln!(file, "    ;; store")?;
            writeln!(file, "    pop rbx")?; // value
            writeln!(file, "    pop rax")?; // address
            writeln!(file, "    mov byte [rax], bl")?; // address
        }
        Token::Syscall1 => {
            writeln!(file, "    ;; syscall1")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    syscall")?;
        }
        Token::Syscall3 => {
            writeln!(file, "    ;; syscall3")?;
            writeln!(file, "    pop rax")?;
            writeln!(file, "    pop rdi")?;
            writeln!(file, "    pop rsi")?;
            writeln!(file, "    pop rdx")?;
            writeln!(file, "    syscall")?;
        }
    }
    Ok(0)
}