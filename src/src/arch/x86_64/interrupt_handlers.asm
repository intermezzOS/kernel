BITS 64

extern interrupt_handler;
extern gdt64.data;


%assign i 0
%rep 34
global isr%+i
%assign i i+1
%endrep

section .interrupt_handlers
%assign i 0
%rep 34
isr%+i:
    mov [number], byte i
    jmp qword push_registers_and_call_handler
%assign i i+1
%endrep

section .text

; Stack must contain rax on top the interrupt frame below. The interrupt
; handler address must then be passed in rax.
push_registers_and_call_handler:
    push rbp
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rsi
    push rdi
    push rdx
    push rcx
    push rbx
    push rax

    mov rsi, rsp
    push rsi
    mov rdi, qword [number]
    push rdi
    
    call qword interrupt_handler

    add rsp, 16 ; Skip interrupt code and reg pointer

    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rdi
    pop rsi
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
    pop rbp

    iretq

number: dq 0
