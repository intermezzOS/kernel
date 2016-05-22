BITS 64

global pop_registers_and_iret

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
global isr34
global isr35
global isr36
global isr37
global isr38
global isr39
global isr40
global isr41
global isr42
global isr43
global isr44
global isr45
global isr46
global isr47
global isr48
global isr49
global isr50
global isr51
global isr52
global isr53
global isr54
global isr55
global isr56
global isr57
global isr58
global isr59
global isr60
global isr61
global isr62
global isr63
global isr64
global isr65
global isr66
global isr67
global isr68
global isr69
global isr70
global isr71
global isr72
global isr73
global isr74
global isr75
global isr76
global isr77
global isr78
global isr79
global isr80
global isr81
global isr82
global isr83
global isr84
global isr85
global isr86
global isr87
global isr88
global isr89
global isr90
global isr91
global isr92
global isr93
global isr94
global isr95
global isr96
global isr97
global isr98
global isr99
global isr100
global isr101
global isr102
global isr103
global isr104
global isr105
global isr106
global isr107
global isr108
global isr109
global isr110
global isr111
global isr112
global isr113
global isr114
global isr115
global isr116
global isr117
global isr118
global isr119
global isr120
global isr121
global isr122
global isr123
global isr124
global isr125
global isr126
global isr127
global isr128
global isr129
global isr130
global isr131
global isr132
global isr133
global isr134
global isr135
global isr136
global isr137
global isr138
global isr139
global isr140
global isr141
global isr142
global isr143
global isr144
global isr145
global isr146
global isr147
global isr148
global isr149
global isr150
global isr151
global isr152
global isr153
global isr154
global isr155
global isr156
global isr157
global isr158
global isr159
global isr160
global isr161
global isr162
global isr163
global isr164
global isr165
global isr166
global isr167
global isr168
global isr169
global isr170
global isr171
global isr172
global isr173
global isr174
global isr175
global isr176
global isr177
global isr178
global isr179
global isr180
global isr181
global isr182
global isr183
global isr184
global isr185
global isr186
global isr187
global isr188
global isr189
global isr190
global isr191
global isr192
global isr193
global isr194
global isr195
global isr196
global isr197
global isr198
global isr199
global isr200
global isr201
global isr202
global isr203
global isr204
global isr205
global isr206
global isr207
global isr208
global isr209
global isr210
global isr211
global isr212
global isr213
global isr214
global isr215
global isr216
global isr217
global isr218
global isr219
global isr220
global isr221
global isr222
global isr223
global isr224
global isr225
global isr226
global isr227
global isr228
global isr229
global isr230
global isr231
global isr232
global isr233
global isr234
global isr235
global isr236
global isr237
global isr238
global isr239
global isr240
global isr241
global isr242
global isr243
global isr244
global isr245
global isr246
global isr247
global isr248
global isr249
global isr250
global isr251
global isr252
global isr253
global isr254
global isr255

extern

section .interrupt_handlers

; special handlers

%macro HANDLER_WITH_ERRCODE 2
    isr%1:
        push qword %1
        push rax
        mov rax, %2
        jmp push_registers_and_call_handler
%endmacro

%define H8
HANDLER_WITH_ERRCODE 8, interrupt_handler
%define H8_IST 1 ;double fault stack

%define H10
HANDLER_WITH_ERRCODE 10, interrupt_handler

%define H11
HANDLER_WITH_ERRCODE 11, interrupt_handler

%define H12
HANDLER_WITH_ERRCODE 12, interrupt_handler

%define H13
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

%define H14
isr14:        ; pagefault
    sub rsp, 8      ; make room for cr2 (replaces interrupt number)
    push rax

    add rsp, 16     ; write cr2 on stack before rax
    mov rax, cr2
    push rax
    sub rsp, 8      ; move rsp to tos again

    mov rax, pagefault_handler
    jmp push_registers_and_call_handler

%define H33
isr33:        ; keyboard
    sub rsp, 8      ; room for keyboard code (replaces error code)
    push qword 33   ; interrupt number
    push rax

    mov rax, 0
    in al, 0x60
    mov [rsp + 16], rax

    mov rax, keyboard_handler
    jmp push_registers_and_call_handler


; other handlers (standard)

%macro HANDLER 1
    %ifndef H%1
    isr%1:
        push qword 0 ;dummy error code
        push qword %1
        push rax
        mov rax, interrupt_handler
        jmp push_registers_and_call_handler
    %endif
%endmacro

%assign i 0
%rep 256
    HANDLER i
%assign i i+1
%endrep


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

; The stack address must be passed in rdi.
pop_registers_and_iret:
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
