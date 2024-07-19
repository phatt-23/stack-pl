bits 64
;;;
section .data
    MINUS db '-', 0
    FD_STDOUT db 1
    SYSCALL_WRITE db 1
    ALPHABET_UPPER db "abcdefghijklmnopqrstuvwxyz", 0
;;;
section .bss
;;;
section .text
    global _start
    global print_num
;;;
sys_write_stdout:
    enter 0, 0
    mov rax, 1
    mov rdi, 1
    syscall
    leave
    ret
;;;
print_num:
    enter 0, 0
        cmp rdi, 0
        mov rcx, 1
        je .done1
        jge .positive
        neg rdi
        push rdi
        mov rsi, MINUS
        mov rdx, 1
        call sys_write_stdout
        pop rdi
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
        jl .done2
        cdq
        idiv rbx ;; rax / rbx
        add rdx, '0'
        mov byte [rsp + rcx], dl
        dec rcx
        jmp .loop2
    .done2:
        mov rsi, rsp
        mov rdx, r8
        call sys_write_stdout
    leave
    ret
 
section .text
    global _start:
 
_start:
address_0:
    ;; push op
    push 1
address_1:
    ;; push op
    push 3
address_2:
    ;; le op
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_3:
    ;; if op
    pop rax
    cmp rax, 0
    jz address_22
address_4:
    ;; push op
    push 1234
address_5:
    ;; push op
    push 1234
address_6:
    ;; eq op
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmove rbx, rax
    push  rbx
address_7:
    ;; if op
    pop rax
    cmp rax, 0
    jz address_16
address_8:
    ;; push op
    push 12
address_9:
    ;; push op
    push 23
address_10:
    ;; le op
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_11:
    ;; if op
    pop rax
    cmp rax, 0
    jz address_14
address_12:
    ;; push op
    push 12345
address_13:
    ;; dump op
    pop rdi
    call print_num
address_14:
    ;; end op
address_15:
    ;; else op
    jmp address_18
address_16:
    ;; push op
    push 54321
address_17:
    ;; dump op
    pop rdi
    call print_num
address_18:
    ;; end op
address_19:
    ;; push op
    push 123
address_20:
    ;; dump op
    pop rdi
    call print_num
address_21:
    ;; else op
    jmp address_24
address_22:
    ;; push op
    push 321
address_23:
    ;; dump op
    pop rdi
    call print_num
address_24:
    ;; end op
    ;;; return
    mov rax, 0x3c
    mov rdi, 0
    syscall
