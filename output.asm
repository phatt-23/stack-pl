bits 64
    ;;;
_start:
address_0:
    ;; push str
    push 13
    push str_0
address_1:
    ;; push op
    push 1
address_2:
    ;; push op
    push 1
address_3:
    ;; syscall3
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
address_4:
    ;; push str
    push 8
    push str_1
address_5:
    ;; push op
    push 1
address_6:
    ;; push op
    push 1
address_7:
    ;; syscall3
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
    ;;; return
    mov rax, 60
    mov rdi, 0
    syscall
    ;;;
section .data
    str_0: db 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 10
    str_1: db 70, 111, 111, 10, 66, 97, 114, 10
    ;;;
section .bss
    MEMORY: resb 65024
    ;;;
section .text
    ;;;
    global _start
    global print_num
    global sys_write_stdout
    ;;;
sys_write_stdout:
    enter 0, 0
    mov   rax, 1
    mov   rdi, 1
    syscall
    leave
    ret
    ;;;
print_num:
    enter 0, 0
        cmp  rdi, 0
        mov  rcx, 1
        je   .done1
        jge  .positive
        neg  rdi
        push rdi
        push '-'
        mov  rsi, rsp
        mov  rdx, 1
        call sys_write_stdout
        pop  rdi
        pop  rdi
    .positive:
        mov rax, rdi
        mov rbx, 10
        xor rcx, rcx
    .loop1:
        cmp rax, 0
        jle .done1
        cdq
        idiv rbx
        inc rcx
        jmp .loop1
    .done1:
        inc rcx
        mov r8, rcx
        sub rsp, rcx
        dec rcx
        mov byte [rsp + rcx], 0x0A
        dec rcx
        cmp rdi, 0
        jnz .skip
        mov byte [rsp + rcx], '0'
        jmp .done2
    .skip:
        mov rax, rdi
    .loop2:
        cmp rcx, 0
        jl  .done2
        cdq
        idiv rbx
        add  rdx, '0'
        mov  byte [rsp + rcx], dl
        dec  rcx
        jmp  .loop2
    .done2:
        mov  rsi, rsp
        mov  rdx, r8
        call sys_write_stdout
    leave
    ret
 
