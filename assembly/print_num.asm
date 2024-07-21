bits 64

section .data
    MINUS db '-', 0
    FD_STDOUT db 1
    SYSCALL_WRITE db 1
    ALPHABET_UPPER db "abcdefghijklmnopqrstuvwxyz", 0

section .bss

section .text
    global _start
    global print_num

sys_write_stdout:
    enter 0, 0
    mov rax, 1
    mov rdi, 1
    syscall
    leave
    ret

print_num: ; print integer to stdout
    enter 0, 0

        cmp rdi, 0
        jge .positive

        neg rdi
        push rdi
        
        mov rsi, MINUS
        mov rdx, 1
        call sys_write_stdout

        pop rdi
    .positive:

    ;; get the number of digits
    mov rax, rdi
    mov rbx, 10
    xor rcx, rcx ;; iterator
    .loop1:
        cmp rax, 0
        jle .done1

        cdq
        idiv rbx ;; rax / rbx
        
        inc rcx
        jmp .loop1
    .done1:
        mov r8, rcx ;; store the result

        sub rsp, rcx ;; allocate on stack
        dec rcx
        mov rax, rdi
    .loop2:
        cmp rcx, 0
        jl .done2

        cdq
        idiv rbx ;; rax / rbx
        add rdx, '0'
        
        mov byte [rsp + rcx], dl

        dec rcx
        jmp .loop2
    .done2: 
        xor rdx, rdx

        mov rax, 1
        mov rdi, 1
        mov rsi, rsp
        mov rdx, r8
        syscall

    leave
    ret

_start: ; main function
    call print_num

    mov rax, 0x3c
    mov rdi, 0
    syscall
