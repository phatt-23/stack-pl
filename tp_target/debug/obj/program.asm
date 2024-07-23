bits 64
    ;;;
section .text
    global _start
    ;;;
    _start:
	address_0:	;; push str
	    push 13
	    push str_0
	address_1:	;; push int
	    push 1
	address_2:	;; push int
	    push 1
	address_3:	;; syscall3
	    pop rax
	    pop rdi
	    pop rsi
	    pop rdx
	    syscall
	address_4:	;; push str
	    push 8
	    push str_1
	address_5:	;; push int
	    push 1
	address_6:	;; push int
	    push 1
	address_7:	;; syscall3
	    pop rax
	    pop rdi
	    pop rsi
	    pop rdx
	    syscall
        ;;; return
        mov rax, 60
        mov rdi, 0
        syscall
    ;;; _start
section .data
    str_0: db 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x0A
    str_1: db 0x46, 0x6F, 0x6F, 0x0A, 0x42, 0x61, 0x72, 0x0A
    ;;;
section .bss
    MEMORY: resb 65024
    ;;;
section .text
    global ___print_integer
    ;;;
    ___print_integer:
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
            mov  rax, 1
            mov  rdi, 1
            syscall
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
            mov  rdx, r8
            mov  rsi, rsp
            mov  rdi, 1
            mov  rax, 1
            syscall
        leave
        ret
    ;;; ____print_integer
section .text
    global ___print_character
    ;;;
    ___print_character:
        enter 0, 0
            or   rdi, 0x0a00
            push rdi
            mov  rsi, rsp
            mov  rax, 1
            mov  rdi, 1
            mov  rdx, 2
            syscall
            pop  r8
        leave
        ret
    ;;; ___print_character
 
