BITS 64

extern interrupt_handler;
extern general_protection_fault_handler;
extern pagefault_handler;
extern keyboard_handler;

global isr0
global isr1
global isr2
global isr3
global isr4
global isr5
global isr6
global isr7
global isr8
global isr9
global isr10
global isr11
global isr12
global isr13
global isr14
global isr15
global isr16
global isr17
global isr18
global isr19
global isr20
global isr21
global isr22
global isr23
global isr24
global isr25
global isr26
global isr27
global isr28
global isr29
global isr30
global isr31
global isr32
global isr33

section .interrupt_handlers

isr0:
    push qword 0 ;dummy error code
    push qword 0
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr1:
    push qword 0 ;dummy error code
    push qword 1
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr2:
    push qword 0 ;dummy error code
    push qword 2
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr3:
    push qword 0 ;dummy error code
    push qword 3
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr4:
    push qword 0 ;dummy error code
    push qword 4
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr5:
    push qword 0 ;dummy error code
    push qword 5
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr6:
    push qword 0 ;dummy error code
    push qword 6
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr7:
    push qword 0 ;dummy error code
    push qword 7
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr8:
    push qword 8
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr9:
    push qword 0 ;dummy error code
    push qword 9
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr10:
    push qword 10
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr11:
    push qword 11
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr12:
    push qword 12
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr13:        ; general protection fault
    sub rsp, 8      ; make room for rip (replaces interrupt number)
    push rax

    add rsp, 24     ; copy rip from trapframe to stack before rax
    pop rax
    sub rsp, 16
    push rax
    sub rsp, 8      ; move rsp to tos again

    mov rax, general_protection_fault_handler
    jmp push_registers_and_call_handler

isr14:        ; pagefault
    sub rsp, 8      ; make room for cr2 (replaces interrupt number)
    push rax

    add rsp, 16     ; write cr2 on stack before rax
    mov rax, cr2
    push rax
    sub rsp, 8      ; move rsp to tos again

    mov rax, pagefault_handler
    jmp push_registers_and_call_handler

isr15:
    push qword 0 ;dummy error code
    push qword 15
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr16:
    push qword 0 ;dummy error code
    push qword 16
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr17:
    push qword 0 ;dummy error code
    push qword 17
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr18:
    push qword 0 ;dummy error code
    push qword 18
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr19:
    push qword 0 ;dummy error code
    push qword 19
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr20:
    push qword 0 ;dummy error code
    push qword 20
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr21:
    push qword 0 ;dummy error code
    push qword 21
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr22:
    push qword 0 ;dummy error code
    push qword 22
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr23:
    push qword 0 ;dummy error code
    push qword 23
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr24:
    push qword 0 ;dummy error code
    push qword 24
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr25:
    push qword 0 ;dummy error code
    push qword 25
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr26:
    push qword 0 ;dummy error code
    push qword 26
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr27:
    push qword 0 ;dummy error code
    push qword 27
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr28:
    push qword 0 ;dummy error code
    push qword 28
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr29:
    push qword 0 ;dummy error code
    push qword 29
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr30:
    push qword 0 ;dummy error code
    push qword 30
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr31:
    push qword 0 ;dummy error code
    push qword 31
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr32:
    push qword 0 ;dummy error code
    push qword 32
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr33:        ; keyboard
    sub rsp, 8      ; room for keyboard code (replaces error code)
    push qword 33   ; interrupt number
    push rax

    mov rax, 0
    in al, 0x60
    mov [rsp + 16], rax

    mov rax, keyboard_handler
    jmp push_registers_and_call_handler

section .text

; Stack must contain rax on top the interrupt frame below. The interrupt
; handler address must then be passed in rax.
push_registers_and_call_handler:
    push rbx
    push rcx
    push rdx
    push rbp
    push rsi
    push rdi

    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    sub rsp, 16
    movdqu [rsp], xmm0
    sub rsp, 16
    movdqu [rsp], xmm1
    sub rsp, 16
    movdqu [rsp], xmm2
    sub rsp, 16
    movdqu [rsp], xmm3
    sub rsp, 16
    movdqu [rsp], xmm4
    sub rsp, 16
    movdqu [rsp], xmm5
    sub rsp, 16
    movdqu [rsp], xmm6
    sub rsp, 16
    movdqu [rsp], xmm7
    sub rsp, 16
    movdqu [rsp], xmm8
    sub rsp, 16
    movdqu [rsp], xmm9
    sub rsp, 16
    movdqu [rsp], xmm10
    sub rsp, 16
    movdqu [rsp], xmm11
    sub rsp, 16
    movdqu [rsp], xmm12
    sub rsp, 16
    movdqu [rsp], xmm13
    sub rsp, 16
    movdqu [rsp], xmm14
    sub rsp, 16
    movdqu [rsp], xmm15

    mov rdi, [rsp + 376]    ; interrupt number
    mov rsi, [rsp + 384]    ; error code
    mov rdx, rsp            ; stack pointer

    call rax

    mov rdi, rsp

    mov rsp, rdi

    movdqu  xmm15, [rsp]
    add     rsp, 16
    movdqu  xmm14, [rsp]
    add     rsp, 16
    movdqu  xmm13, [rsp]
    add     rsp, 16
    movdqu  xmm12, [rsp]
    add     rsp, 16
    movdqu  xmm11, [rsp]
    add     rsp, 16
    movdqu  xmm10, [rsp]
    add     rsp, 16
    movdqu  xmm9, [rsp]
    add     rsp, 16
    movdqu  xmm8, [rsp]
    add     rsp, 16
    movdqu  xmm7, [rsp]
    add     rsp, 16
    movdqu  xmm6, [rsp]
    add     rsp, 16
    movdqu  xmm5, [rsp]
    add     rsp, 16
    movdqu  xmm4, [rsp]
    add     rsp, 16
    movdqu  xmm3, [rsp]
    add     rsp, 16
    movdqu  xmm2, [rsp]
    add     rsp, 16
    movdqu  xmm1, [rsp]
    add     rsp, 16
    movdqu  xmm0, [rsp]
    add     rsp, 16

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8

    pop rdi
    pop rsi
    pop rbp
    pop rdx
    pop rcx
    pop rbx
    pop rax

    add rsp, 16 ;remove interrupt number and error code

    iretq
