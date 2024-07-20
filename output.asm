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
    ;; mem
    push MEMORY
address_1:
    ;; push op
    push 98
address_2:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_3:
    ;; push op
    push 1
address_4:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_5:
    ;; push op
    push 0
address_6:
    ;; while
    ;  ignore
address_7:
    ;; dup
    pop  rax
    push rax
    push rax
address_8:
    ;; push op
    push 98
address_9:
    ;; le
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_10:
    ;; do
    pop  rax
    cmp  rax, 0
    jz   address_101
address_11:
    ;; push op
    push 0
address_12:
    ;; while
    ;  ignore
address_13:
    ;; dup
    pop  rax
    push rax
    push rax
address_14:
    ;; push op
    push 100
address_15:
    ;; le
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_16:
    ;; do
    pop  rax
    cmp  rax, 0
    jz   address_41
address_17:
    ;; dup
    pop  rax
    push rax
    push rax
address_18:
    ;; mem
    push MEMORY
address_19:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_20:
    ;; load
    pop   rax
    xor   rbx, rbx
    mov   bl, byte [rax]
    push  rbx
address_21:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_30
address_22:
    ;; dup
    pop  rax
    push rax
    push rax
address_23:
    ;; mem
    push MEMORY
address_24:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_25:
    ;; push op
    push 100
address_26:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_27:
    ;; push op
    push 42
address_28:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_29:
    ;; else
    jmp address_37
address_30:
    ;; dup
    pop  rax
    push rax
    push rax
address_31:
    ;; mem
    push MEMORY
address_32:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_33:
    ;; push op
    push 100
address_34:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_35:
    ;; push op
    push 32
address_36:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_37:
    ;; end
address_38:
    ;; push op
    push 1
address_39:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_40:
    ;; end
    jmp address_12
address_41:
    ;; mem
    push MEMORY
address_42:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_43:
    ;; push op
    push 100
address_44:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_45:
    ;; push op
    push 10
address_46:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_47:
    ;; push op
    push 101
address_48:
    ;; mem
    push MEMORY
address_49:
    ;; push op
    push 100
address_50:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_51:
    ;; push op
    push 1
address_52:
    ;; push op
    push 1
address_53:
    ;; syscall3
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
address_54:
    ;; mem
    push MEMORY
address_55:
    ;; load
    pop   rax
    xor   rbx, rbx
    mov   bl, byte [rax]
    push  rbx
address_56:
    ;; push op
    push 1
address_57:
    ;; shift left
    pop   rcx
    pop   rbx
    shl   rbx, cl
    push  rbx
address_58:
    ;; mem
    push MEMORY
address_59:
    ;; push op
    push 1
address_60:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_61:
    ;; load
    pop   rax
    xor   rbx, rbx
    mov   bl, byte [rax]
    push  rbx
address_62:
    ;; bit or
    pop   rax
    pop   rbx
    or    rbx, rax
    push  rbx
address_63:
    ;; push op
    push 1
address_64:
    ;; while
    ;  ignore
address_65:
    ;; dup
    pop  rax
    push rax
    push rax
address_66:
    ;; push op
    push 98
address_67:
    ;; le
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_68:
    ;; do
    pop  rax
    cmp  rax, 0
    jz   address_96
address_69:
    ;; swap
    pop  rax
    pop  rbx
    push rax
    push rbx
address_70:
    ;; push op
    push 1
address_71:
    ;; shift left
    pop   rcx
    pop   rbx
    shl   rbx, cl
    push  rbx
address_72:
    ;; push op
    push 7
address_73:
    ;; bit and
    pop   rax
    pop   rbx
    and   rbx, rax
    push  rbx
address_74:
    ;; over
    pop  rax
    pop  rbx
    push rbx
    push rax
    push rbx
address_75:
    ;; mem
    push MEMORY
address_76:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_77:
    ;; push op
    push 1
address_78:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_79:
    ;; load
    pop   rax
    xor   rbx, rbx
    mov   bl, byte [rax]
    push  rbx
address_80:
    ;; bit or
    pop   rax
    pop   rbx
    or    rbx, rax
    push  rbx
address_81:
    ;; dup 2
    pop  rax
    pop  rbx
    push rbx
    push rax
    push rbx
    push rax
address_82:
    ;; push op
    push 110
address_83:
    ;; swap
    pop  rax
    pop  rbx
    push rax
    push rbx
address_84:
    ;; shift right
    pop   rcx
    pop   rbx
    shr   rbx, cl
    push  rbx
address_85:
    ;; push op
    push 1
address_86:
    ;; bit and
    pop   rax
    pop   rbx
    and   rbx, rax
    push  rbx
address_87:
    ;; swap
    pop  rax
    pop  rbx
    push rax
    push rbx
address_88:
    ;; mem
    push MEMORY
address_89:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_90:
    ;; swap
    pop  rax
    pop  rbx
    push rax
    push rbx
address_91:
    ;; store
    pop rbx
    pop rax
    mov byte [rax], bl
address_92:
    ;; swap
    pop  rax
    pop  rbx
    push rax
    push rbx
address_93:
    ;; push op
    push 1
address_94:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_95:
    ;; end
    jmp address_64
address_96:
    ;; drop
    add rsp, 8
address_97:
    ;; drop
    add rsp, 8
address_98:
    ;; push op
    push 1
address_99:
    ;; plus
    pop  rax
    pop  rbx
    add  rbx, rax
    push rbx
address_100:
    ;; end
    jmp address_6
address_101:
    ;; drop
    add rsp, 8
    ;;; return
    mov rax, [SYSCALL_EXIT]
    mov rdi, 0
    syscall
