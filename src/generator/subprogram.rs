use std::io::Write;

pub const SUBPROGRAM_IDENTIFIER_PRINT_INTEGER: &str = "___print_integer";
pub const SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER: &str = "___print_character";

pub fn write_print_num(file: &mut std::fs::File) -> Result<i32, std::io::Error> {
    writeln!(file, "section .text")?;
    writeln!(file, "    global {}", SUBPROGRAM_IDENTIFIER_PRINT_INTEGER)?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "    {}:", SUBPROGRAM_IDENTIFIER_PRINT_INTEGER)?;
    writeln!(file, "        enter 0, 0")?;
    writeln!(file, "            cmp  rdi, 0")?;
    writeln!(file, "            mov  rcx, 1")?;
    writeln!(file, "            je   .done1")?;
    writeln!(file, "            jge  .positive")?;
    writeln!(file, "            neg  rdi")?;
    writeln!(file, "            push rdi")?;
    writeln!(file, "            push '-'")?;
    writeln!(file, "            mov  rsi, rsp")?;
    writeln!(file, "            mov  rdx, 1")?;
    writeln!(file, "            mov  rax, 1")?;
    writeln!(file, "            mov  rdi, 1")?;
    writeln!(file, "            syscall")?;
    writeln!(file, "            pop  rdi")?; // dropping '-'
    writeln!(file, "            pop  rdi")?;
    writeln!(file, "        .positive:")?;
    writeln!(file, "            mov rax, rdi")?;
    writeln!(file, "            mov rbx, 10")?;
    writeln!(file, "            xor rcx, rcx")?;
    writeln!(file, "        .loop1:")?;
    writeln!(file, "            cmp rax, 0")?;
    writeln!(file, "            jle .done1")?;
    writeln!(file, "            cdq")?;
    writeln!(file, "            idiv rbx")?;
    writeln!(file, "            inc rcx")?;
    writeln!(file, "            jmp .loop1")?;
    writeln!(file, "        .done1:")?;
    writeln!(file, "            inc rcx")?;
    writeln!(file, "            mov r8, rcx")?;
    writeln!(file, "            sub rsp, rcx")?;
    writeln!(file, "            dec rcx")?;
    writeln!(file, "            mov byte [rsp + rcx], 0x0A")?;
    writeln!(file, "            dec rcx")?;
    writeln!(file, "            cmp rdi, 0")?;
    writeln!(file, "            jnz .skip")?;
    writeln!(file, "            mov byte [rsp + rcx], '0'")?;
    writeln!(file, "            jmp .done2")?;
    writeln!(file, "        .skip:")?;
    writeln!(file, "            mov rax, rdi")?;
    writeln!(file, "        .loop2:")?;
    writeln!(file, "            cmp rcx, 0")?;
    writeln!(file, "            jl  .done2")?;
    writeln!(file, "            cdq")?;
    writeln!(file, "            idiv rbx")?;
    writeln!(file, "            add  rdx, '0'")?;
    writeln!(file, "            mov  byte [rsp + rcx], dl")?;
    writeln!(file, "            dec  rcx")?;
    writeln!(file, "            jmp  .loop2")?;
    writeln!(file, "        .done2:")?;
    writeln!(file, "            mov  rdx, r8")?;
    writeln!(file, "            mov  rsi, rsp")?;
    writeln!(file, "            mov  rdi, 1")?;
    writeln!(file, "            mov  rax, 1")?;
    writeln!(file, "            syscall")?;
    writeln!(file, "        leave")?;
    writeln!(file, "        ret")?;
    writeln!(file, "    ;;; _{}", SUBPROGRAM_IDENTIFIER_PRINT_INTEGER)?;

    Ok(0)
}

pub fn write_print_char(file: &mut std::fs::File) -> Result<i32, std::io::Error> {
    writeln!(file, "section .text")?;
    writeln!(file, "    global {}", SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?;
    writeln!(file, "    ;;;")?;
    writeln!(file, "    {}:", SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?;
    writeln!(file, "        enter 0, 0")?;
    writeln!(file, "            or   rdi, 0x0a00")?; // adding new line
    writeln!(file, "            push rdi")?;
    writeln!(file, "            mov  rsi, rsp")?;
    writeln!(file, "            mov  rax, 1")?;
    writeln!(file, "            mov  rdi, 1")?;
    writeln!(file, "            mov  rdx, 2")?;
    writeln!(file, "            syscall")?;   
    writeln!(file, "            pop  r8")?;
    writeln!(file, "        leave")?;
    writeln!(file, "        ret")?;
    writeln!(file, "    ;;; {}", SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?;

    Ok(0)
}
