use std::io::Write;

pub const SUBPROGRAM_IDENTIFIER_PRINT_INTEGER_64BIT: &str = "__i64_print";
pub const SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER: &str = "__char_print";

pub fn write_print_num(file: &mut std::fs::File) -> Result<i32, std::io::Error> {
    writeln!(file, "        global {}", SUBPROGRAM_IDENTIFIER_PRINT_INTEGER_64BIT)?;
    writeln!(file, "{}:", SUBPROGRAM_IDENTIFIER_PRINT_INTEGER_64BIT)?;
    writeln!(file, "        push rbp")?;
    writeln!(file, "        mov rbp, rsp")?;
    writeln!(file, "        sub rsp, 24")?;
    writeln!(file, "                xor rcx, rcx")?;
    writeln!(file, "                cmp rdi, 0")?;
    writeln!(file, "                jz .zero")?;
    writeln!(file, "                jg .positive")?;
    writeln!(file, "                neg rdi")?;
    writeln!(file, "                mov byte [rsp + rcx], '-'")?;
    writeln!(file, "                inc rcx")?;
    writeln!(file, "        .positive:")?;
    writeln!(file, "                mov rbx, 10")?;
    writeln!(file, "                mov rax, rdi")?;
    writeln!(file, "        .get_digit:")?;
    writeln!(file, "                cmp rax, 0")?;
    writeln!(file, "                jle .add_newline")?;
    writeln!(file, "                cdq")?;
    writeln!(file, "                xor rdx, rdx")?;
    writeln!(file, "                div rbx")?;
    writeln!(file, "                add rdx, '0'")?;
    writeln!(file, "                mov byte [rsp + rcx], dl")?;
    writeln!(file, "                inc rcx")?;
    writeln!(file, "                jmp .get_digit")?;
    writeln!(file, "        .add_newline:")?;
    writeln!(file, "                mov word [rsp + rcx], 0x000A")?;
    writeln!(file, "                inc rcx")?;
    writeln!(file, "                lea rsi, [rsp]")?;
    writeln!(file, "                lea rdi, [rsp + rcx - 2]")?;
    writeln!(file, "                cmp byte [rsp], '-'")?;
    writeln!(file, "                je .exclude_sign")?;
    writeln!(file, "        .reverse:")?;
    writeln!(file, "                cmp rsi, rdi")?;
    writeln!(file, "                jge .write")?;
    writeln!(file, "                mov al, byte [rdi]")?;
    writeln!(file, "                mov ah, byte [rsi]")?;
    writeln!(file, "                mov byte [rdi], ah")?;
    writeln!(file, "                mov byte [rsi], al")?;
    writeln!(file, "                inc rsi")?;
    writeln!(file, "                dec rdi")?;
    writeln!(file, "                jmp .reverse")?;
    writeln!(file, "        .exclude_sign:")?;
    writeln!(file, "                lea rsi, [rsp + 1]")?;
    writeln!(file, "                jmp .reverse")?;
    writeln!(file, "        .write:")?;
    writeln!(file, "                mov rax, 1")?;
    writeln!(file, "                mov rdi, 1")?;
    writeln!(file, "                mov rsi, rsp")?;
    writeln!(file, "                mov rdx, rcx")?;
    writeln!(file, "                syscall")?;
    writeln!(file, "                mov rsp, rbp")?;
    writeln!(file, "                pop rbp")?;
    writeln!(file, "                ret")?;
    writeln!(file, "        .zero:")?;
    writeln!(file, "                mov byte [rsp + rcx], '0'")?;
    writeln!(file, "                inc rcx")?;
    writeln!(file, "                mov word [rsp + rcx], 0x000A")?;
    writeln!(file, "                inc rcx")?;
    writeln!(file, "                jmp .write")?;
    writeln!(file, ";;;{}", SUBPROGRAM_IDENTIFIER_PRINT_INTEGER_64BIT)?;
    Ok(0)
}

pub fn write_print_char(file: &mut std::fs::File) -> Result<i32, std::io::Error> {
    writeln!(file, "        global {}", SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?;
    writeln!(file, "{}:", SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?;
    writeln!(file, "        push rbp")?;
    writeln!(file, "        mov rbp, rsp")?;
    writeln!(file, "                or   rdi, 0x0a00")?; // adding new line
    writeln!(file, "                push rdi")?;
    writeln!(file, "                mov  rsi, rsp")?;
    writeln!(file, "                mov  rax, 1")?;
    writeln!(file, "                mov  rdi, 1")?;
    writeln!(file, "                mov  rdx, 2")?;
    writeln!(file, "                syscall")?;   
    writeln!(file, "                pop  r8")?;
    writeln!(file, "        leave")?;
    writeln!(file, "        ret")?;
    writeln!(file, ";;;{}", SUBPROGRAM_IDENTIFIER_PRINT_CHARACTER)?;

    Ok(0)
}
