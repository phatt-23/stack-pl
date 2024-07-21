bits 64
    ;;;
_start:
address_0:
    ;; mem
    push MEMORY
    pop rax
    add rax, 1024
    push rax
address_1:
    ;; dup
    pop  rax
    push rax
    push rax
address_2:
    ;; push op
    push 72
address_3:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_4:
    ;; push op
    push 1
address_5:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_6:
    ;; dup
    pop  rax
    push rax
    push rax
address_7:
    ;; push op
    push 101
address_8:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_9:
    ;; push op
    push 1
address_10:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_11:
    ;; dup
    pop  rax
    push rax
    push rax
address_12:
    ;; push op
    push 108
address_13:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_14:
    ;; push op
    push 1
address_15:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_16:
    ;; dup
    pop  rax
    push rax
    push rax
address_17:
    ;; push op
    push 108
address_18:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_19:
    ;; push op
    push 1
address_20:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_21:
    ;; dup
    pop  rax
    push rax
    push rax
address_22:
    ;; push op
    push 111
address_23:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_24:
    ;; push op
    push 1
address_25:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_26:
    ;; dup
    pop  rax
    push rax
    push rax
address_27:
    ;; push op
    push 10
address_28:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_29:
    ;; dup
    pop  rax
    push rax
    push rax
address_30:
    ;; mem
    push MEMORY
    pop rax
    add rax, 1024
    push rax
address_31:
    ;; push op
    push 1
address_32:
    ;; push op
    push 1
address_33:
    ;; syscall3
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
address_34:
    ;; push op
    push 123
address_35:
    ;; push op
    push 60
address_36:
    ;; syscall1
    pop rax
    pop rdi
    syscall
    ;;; return
    mov rax, 60
    mov rdi, 0
    syscall
    ;;;
section .data
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
 
