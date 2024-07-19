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
    ;; le
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_3:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_22
address_4:
    ;; push op
    push 1234
address_5:
    ;; push op
    push 1234
address_6:
    ;; eq
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmove rbx, rax
    push  rbx
address_7:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_16
address_8:
    ;; push op
    push 12
address_9:
    ;; push op
    push 23
address_10:
    ;; le
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_11:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_14
address_12:
    ;; push op
    push 12345
address_13:
    ;; dump
    pop rdi
    call print_num
address_14:
    ;; end
address_15:
    ;; else
    jmp address_18
address_16:
    ;; push op
    push 54321
address_17:
    ;; dump
    pop rdi
    call print_num
address_18:
    ;; end
address_19:
    ;; push op
    push 123
address_20:
    ;; dump
    pop rdi
    call print_num
address_21:
    ;; else
    jmp address_24
address_22:
    ;; push op
    push 321
address_23:
    ;; dump
    pop rdi
    call print_num
address_24:
    ;; end
address_25:
    ;; push op
    push 1
address_26:
    ;; push op
    push 2
address_27:
    ;; eq
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmove rbx, rax
    push  rbx
address_28:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_52
address_29:
    ;; push op
    push 3
address_30:
    ;; push op
    push 3
address_31:
    ;; eq
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmove rbx, rax
    push  rbx
address_32:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_45
address_33:
    ;; push op
    push 1
address_34:
    ;; dump
    pop rdi
    call print_num
address_35:
    ;; push op
    push 3
address_36:
    ;; push op
    push 4
address_37:
    ;; gr
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovg rbx, rax
    push  rbx
address_38:
    ;; push op
    push 2
address_39:
    ;; dump
    pop rdi
    call print_num
address_40:
    ;; push op
    push 45
address_41:
    ;; push op
    push 3
address_42:
    ;; gr
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovg rbx, rax
    push  rbx
address_43:
    ;; push op
    push 3
address_44:
    ;; dump
    pop rdi
    call print_num
address_45:
    ;; end
address_46:
    ;; push op
    push 1
address_47:
    ;; push op
    push 1
address_48:
    ;; eq
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmove rbx, rax
    push  rbx
address_49:
    ;; push op
    push 4
address_50:
    ;; dump
    pop rdi
    call print_num
address_51:
    ;; else
    jmp address_54
address_52:
    ;; push op
    push 5
address_53:
    ;; dump
    pop rdi
    call print_num
address_54:
    ;; end
address_55:
    ;; push op
    push 1
address_56:
    ;; push op
    push 1
address_57:
    ;; gr
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovg rbx, rax
    push  rbx
address_58:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_62
address_59:
    ;; push op
    push 111
address_60:
    ;; dump
    pop rdi
    call print_num
address_61:
    ;; else
    jmp address_80
address_62:
    ;; push op
    push 1
address_63:
    ;; push op
    push 1
address_64:
    ;; le
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmovl rbx, rax
    push  rbx
address_65:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_69
address_66:
    ;; push op
    push 222
address_67:
    ;; dump
    pop rdi
    call print_num
address_68:
    ;; else
    jmp address_79
address_69:
    ;; push op
    push 1
address_70:
    ;; push op
    push 1
address_71:
    ;; eq
    pop   rax
    pop   rbx
    cmp   rbx, rax
    mov   rbx, 0
    mov   rax, 1
    cmove rbx, rax
    push  rbx
address_72:
    ;; if
    pop rax
    cmp rax, 0
    jz  address_76
address_73:
    ;; push op
    push 333
address_74:
    ;; dump
    pop rdi
    call print_num
address_75:
    ;; else
    jmp address_78
address_76:
    ;; push op
    push 444
address_77:
    ;; dump
    pop rdi
    call print_num
address_78:
    ;; end
address_79:
    ;; end
address_80:
    ;; end
    ;;; return
    mov rax, 0x3c
    mov rdi, 0
    syscall
