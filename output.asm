bits 64
    ;;;
section .data
    MINUS:          db '-', 0x00
    FD_STDOUT:      dq 1
    SYSCALL_WRITE:  dq 1
    SYSCALL_EXIT:   dq 60
    ALPHABET_UPPER: db "abcdefghijklmnopqrstuvwxyz", 0x00
    ;;;
section .bss
    MEMORY: resb 1000000
    ;;;
section .text
    ;;;
    global _start
    global print_num
    global sys_write_stdout
    ;;;
sys_write_stdout:
    enter 0, 0
    mov   rax, [SYSCALL_WRITE]
    mov   rdi, [FD_STDOUT]
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
        mov  rsi, MINUS
        mov  rdx, 1
        call sys_write_stdout
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
 
_start:
address_0:
    ;; push op
    push 30
address_1:
    ;; push op
    push 0
address_2:
    ;; while
    ;  ignore
address_3:
    ;; dup 2
    pop  rax
    pop  rbx
    push rbx
    push rax
    push rbx
    push rax
address_4:
    ;; gr
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovg rbx, rax
    push  rbx
address_5:
    ;; do
    pop  rax
    cmp  rax, 0
    jz   address_56
address_6:
    ;; push op
    push 30
address_7:
    ;; push op
    push 0
address_8:
    ;; while
    ;  ignore
address_9:
    ;; dup 2
    pop  rax
    pop  rbx
    push rbx
    push rax
    push rbx
    push rax
address_10:
    ;; gr
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovg rbx, rax
    push  rbx
address_11:
    ;; do
    pop  rax
    cmp  rax, 0
    jz   address_39
address_12:
    ;; dup
    pop  rax
    push rax
    push rax
address_13:
    ;; mem
    push MEMORY
address_14:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_15:
    ;; load
    pop   rax
    xor   rbx, rbx
    mov   bl, byte [rax]
    push  rbx
address_16:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_23
address_17:
    ;; mem
    push MEMORY
address_18:
    ;; push op
    push 30
address_19:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_20:
    ;; push op
    push 42
address_21:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_22:
    ;; else
    jmp address_28
address_23:
    ;; mem
    push MEMORY
address_24:
    ;; push op
    push 30
address_25:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_26:
    ;; push op
    push 42
address_27:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_28:
    ;; end
address_29:
    ;; push op
    push 1
address_30:
    ;; mem
    push MEMORY
address_31:
    ;; push op
    push 30
address_32:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_33:
    ;; push op
    push 1
address_34:
    ;; push op
    push 1
address_35:
    ;; syscall3
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
address_36:
    ;; push op
    push 1
address_37:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_38:
    ;; end
    jmp address_8
address_39:
    ;; drop
    add rsp, 8
address_40:
    ;; drop
    add rsp, 8
address_41:
    ;; mem
    push MEMORY
address_42:
    ;; push op
    push 30
address_43:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_44:
    ;; push op
    push 10
address_45:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_46:
    ;; push op
    push 1
address_47:
    ;; mem
    push MEMORY
address_48:
    ;; push op
    push 30
address_49:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_50:
    ;; push op
    push 1
address_51:
    ;; push op
    push 1
address_52:
    ;; syscall3
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
address_53:
    ;; push op
    push 1
address_54:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_55:
    ;; end
    jmp address_2
address_56:
    ;; drop
    add rsp, 8
address_57:
    ;; drop
    add rsp, 8
address_58:
    ;; push op
    push 0
    ;;; return
    mov rax, [SYSCALL_EXIT]
    mov rdi, 0
    syscall
