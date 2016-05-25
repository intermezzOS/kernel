BITS 64

global load_idt
extern IDT_POINTER

section .text
load_idt:
    lidt [IDT_POINTER]
    ret

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

section .interrupt_handlers

%define H8_IST 1 ;double fault stack

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

isr34:
    push qword 0 ;dummy error code
    push qword 32
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr35:
    push qword 0 ;dummy error code
    push qword 33
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr36:
    push qword 0 ;dummy error code
    push qword 34
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr37:
    push qword 0 ;dummy error code
    push qword 35
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr38:
    push qword 0 ;dummy error code
    push qword 36
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr39:
    push qword 0 ;dummy error code
    push qword 37
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr40:
    push qword 0 ;dummy error code
    push qword 38
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr41:
    push qword 0 ;dummy error code
    push qword 39
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr42:
    push qword 0 ;dummy error code
    push qword 40
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr43:
    push qword 0 ;dummy error code
    push qword 41
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr44:
    push qword 0 ;dummy error code
    push qword 42
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr45:
    push qword 0 ;dummy error code
    push qword 43
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr46:
    push qword 0 ;dummy error code
    push qword 44
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr47:
    push qword 0 ;dummy error code
    push qword 45
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr48:
    push qword 0 ;dummy error code
    push qword 46
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr49:
    push qword 0 ;dummy error code
    push qword 47
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr50:
    push qword 0 ;dummy error code
    push qword 48
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr51:
    push qword 0 ;dummy error code
    push qword 49
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr52:
    push qword 0 ;dummy error code
    push qword 50
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr53:
    push qword 0 ;dummy error code
    push qword 51
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr54:
    push qword 0 ;dummy error code
    push qword 52
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr55:
    push qword 0 ;dummy error code
    push qword 53
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr56:
    push qword 0 ;dummy error code
    push qword 54
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr57:
    push qword 0 ;dummy error code
    push qword 55
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr58:
    push qword 0 ;dummy error code
    push qword 56
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr59:
    push qword 0 ;dummy error code
    push qword 57
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr60:
    push qword 0 ;dummy error code
    push qword 58
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr61:
    push qword 0 ;dummy error code
    push qword 59
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr62:
    push qword 0 ;dummy error code
    push qword 60
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr63:
    push qword 0 ;dummy error code
    push qword 61
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr64:
    push qword 0 ;dummy error code
    push qword 62
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr65:
    push qword 0 ;dummy error code
    push qword 63
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr66:
    push qword 0 ;dummy error code
    push qword 64
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr67:
    push qword 0 ;dummy error code
    push qword 65
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr68:
    push qword 0 ;dummy error code
    push qword 66
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr69:
    push qword 0 ;dummy error code
    push qword 67
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr70:
    push qword 0 ;dummy error code
    push qword 68
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr71:
    push qword 0 ;dummy error code
    push qword 69
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr72:
    push qword 0 ;dummy error code
    push qword 70
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr73:
    push qword 0 ;dummy error code
    push qword 71
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr74:
    push qword 0 ;dummy error code
    push qword 72
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr75:
    push qword 0 ;dummy error code
    push qword 73
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr76:
    push qword 0 ;dummy error code
    push qword 74
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr77:
    push qword 0 ;dummy error code
    push qword 75
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr78:
    push qword 0 ;dummy error code
    push qword 76
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr79:
    push qword 0 ;dummy error code
    push qword 77
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr80:
    push qword 0 ;dummy error code
    push qword 78
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr81:
    push qword 0 ;dummy error code
    push qword 79
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr82:
    push qword 0 ;dummy error code
    push qword 80
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr83:
    push qword 0 ;dummy error code
    push qword 81
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr84:
    push qword 0 ;dummy error code
    push qword 82
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr85:
    push qword 0 ;dummy error code
    push qword 83
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr86:
    push qword 0 ;dummy error code
    push qword 84
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr87:
    push qword 0 ;dummy error code
    push qword 85
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr88:
    push qword 0 ;dummy error code
    push qword 86
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr89:
    push qword 0 ;dummy error code
    push qword 87
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr90:
    push qword 0 ;dummy error code
    push qword 88
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr91:
    push qword 0 ;dummy error code
    push qword 89
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr92:
    push qword 0 ;dummy error code
    push qword 90
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr93:
    push qword 0 ;dummy error code
    push qword 91
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr94:
    push qword 0 ;dummy error code
    push qword 92
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr95:
    push qword 0 ;dummy error code
    push qword 93
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr96:
    push qword 0 ;dummy error code
    push qword 94
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr97:
    push qword 0 ;dummy error code
    push qword 95
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr98:
    push qword 0 ;dummy error code
    push qword 96
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr99:
    push qword 0 ;dummy error code
    push qword 97
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr100:
    push qword 0 ;dummy error code
    push qword 98
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr101:
    push qword 0 ;dummy error code
    push qword 99
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr102:
    push qword 0 ;dummy error code
    push qword 100
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr103:
    push qword 0 ;dummy error code
    push qword 101
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr104:
    push qword 0 ;dummy error code
    push qword 102
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr105:
    push qword 0 ;dummy error code
    push qword 103
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr106:
    push qword 0 ;dummy error code
    push qword 104
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr107:
    push qword 0 ;dummy error code
    push qword 105
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr108:
    push qword 0 ;dummy error code
    push qword 106
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr109:
    push qword 0 ;dummy error code
    push qword 107
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr110:
    push qword 0 ;dummy error code
    push qword 108
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr111:
    push qword 0 ;dummy error code
    push qword 109
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr112:
    push qword 0 ;dummy error code
    push qword 110
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr113:
    push qword 0 ;dummy error code
    push qword 111
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr114:
    push qword 0 ;dummy error code
    push qword 112
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr115:
    push qword 0 ;dummy error code
    push qword 113
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr116:
    push qword 0 ;dummy error code
    push qword 114
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr117:
    push qword 0 ;dummy error code
    push qword 115
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr118:
    push qword 0 ;dummy error code
    push qword 116
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr119:
    push qword 0 ;dummy error code
    push qword 117
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr120:
    push qword 0 ;dummy error code
    push qword 118
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr121:
    push qword 0 ;dummy error code
    push qword 119
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr122:
    push qword 0 ;dummy error code
    push qword 120
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr123:
    push qword 0 ;dummy error code
    push qword 121
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr124:
    push qword 0 ;dummy error code
    push qword 122
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr125:
    push qword 0 ;dummy error code
    push qword 123
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr126:
    push qword 0 ;dummy error code
    push qword 124
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr127:
    push qword 0 ;dummy error code
    push qword 125
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr128:
    push qword 0 ;dummy error code
    push qword 126
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr129:
    push qword 0 ;dummy error code
    push qword 127
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr130:
    push qword 0 ;dummy error code
    push qword 128
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr131:
    push qword 0 ;dummy error code
    push qword 129
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr132:
    push qword 0 ;dummy error code
    push qword 130
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr133:
    push qword 0 ;dummy error code
    push qword 131
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr134:
    push qword 0 ;dummy error code
    push qword 132
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr135:
    push qword 0 ;dummy error code
    push qword 133
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr136:
    push qword 0 ;dummy error code
    push qword 134
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr137:
    push qword 0 ;dummy error code
    push qword 135
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr138:
    push qword 0 ;dummy error code
    push qword 136
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr139:
    push qword 0 ;dummy error code
    push qword 137
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr140:
    push qword 0 ;dummy error code
    push qword 138
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr141:
    push qword 0 ;dummy error code
    push qword 139
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr142:
    push qword 0 ;dummy error code
    push qword 140
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr143:
    push qword 0 ;dummy error code
    push qword 141
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr144:
    push qword 0 ;dummy error code
    push qword 142
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr145:
    push qword 0 ;dummy error code
    push qword 143
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr146:
    push qword 0 ;dummy error code
    push qword 144
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr147:
    push qword 0 ;dummy error code
    push qword 145
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr148:
    push qword 0 ;dummy error code
    push qword 146
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr149:
    push qword 0 ;dummy error code
    push qword 147
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr150:
    push qword 0 ;dummy error code
    push qword 148
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr151:
    push qword 0 ;dummy error code
    push qword 149
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr152:
    push qword 0 ;dummy error code
    push qword 150
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr153:
    push qword 0 ;dummy error code
    push qword 151
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr154:
    push qword 0 ;dummy error code
    push qword 152
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr155:
    push qword 0 ;dummy error code
    push qword 153
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr156:
    push qword 0 ;dummy error code
    push qword 154
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr157:
    push qword 0 ;dummy error code
    push qword 155
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr158:
    push qword 0 ;dummy error code
    push qword 156
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr159:
    push qword 0 ;dummy error code
    push qword 157
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr160:
    push qword 0 ;dummy error code
    push qword 158
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr161:
    push qword 0 ;dummy error code
    push qword 159
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr162:
    push qword 0 ;dummy error code
    push qword 160
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr163:
    push qword 0 ;dummy error code
    push qword 161
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr164:
    push qword 0 ;dummy error code
    push qword 162
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr165:
    push qword 0 ;dummy error code
    push qword 163
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr166:
    push qword 0 ;dummy error code
    push qword 164
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr167:
    push qword 0 ;dummy error code
    push qword 165
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr168:
    push qword 0 ;dummy error code
    push qword 166
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr169:
    push qword 0 ;dummy error code
    push qword 167
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr170:
    push qword 0 ;dummy error code
    push qword 168
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr171:
    push qword 0 ;dummy error code
    push qword 169
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr172:
    push qword 0 ;dummy error code
    push qword 170
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr173:
    push qword 0 ;dummy error code
    push qword 171
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr174:
    push qword 0 ;dummy error code
    push qword 172
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr175:
    push qword 0 ;dummy error code
    push qword 173
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr176:
    push qword 0 ;dummy error code
    push qword 174
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr177:
    push qword 0 ;dummy error code
    push qword 175
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr178:
    push qword 0 ;dummy error code
    push qword 176
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr179:
    push qword 0 ;dummy error code
    push qword 177
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr180:
    push qword 0 ;dummy error code
    push qword 178
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr181:
    push qword 0 ;dummy error code
    push qword 179
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr182:
    push qword 0 ;dummy error code
    push qword 180
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr183:
    push qword 0 ;dummy error code
    push qword 181
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr184:
    push qword 0 ;dummy error code
    push qword 182
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr185:
    push qword 0 ;dummy error code
    push qword 183
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr186:
    push qword 0 ;dummy error code
    push qword 184
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr187:
    push qword 0 ;dummy error code
    push qword 185
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr188:
    push qword 0 ;dummy error code
    push qword 186
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr189:
    push qword 0 ;dummy error code
    push qword 187
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr190:
    push qword 0 ;dummy error code
    push qword 188
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr191:
    push qword 0 ;dummy error code
    push qword 189
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr192:
    push qword 0 ;dummy error code
    push qword 190
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr193:
    push qword 0 ;dummy error code
    push qword 191
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr194:
    push qword 0 ;dummy error code
    push qword 192
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr195:
    push qword 0 ;dummy error code
    push qword 193
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr196:
    push qword 0 ;dummy error code
    push qword 194
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr197:
    push qword 0 ;dummy error code
    push qword 195
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr198:
    push qword 0 ;dummy error code
    push qword 196
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr199:
    push qword 0 ;dummy error code
    push qword 197
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr200:
    push qword 0 ;dummy error code
    push qword 198
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr201:
    push qword 0 ;dummy error code
    push qword 199
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr202:
    push qword 0 ;dummy error code
    push qword 200
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr203:
    push qword 0 ;dummy error code
    push qword 201
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr204:
    push qword 0 ;dummy error code
    push qword 202
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr205:
    push qword 0 ;dummy error code
    push qword 203
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr206:
    push qword 0 ;dummy error code
    push qword 204
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr207:
    push qword 0 ;dummy error code
    push qword 205
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr208:
    push qword 0 ;dummy error code
    push qword 206
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr209:
    push qword 0 ;dummy error code
    push qword 207
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr210:
    push qword 0 ;dummy error code
    push qword 208
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr211:
    push qword 0 ;dummy error code
    push qword 209
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr212:
    push qword 0 ;dummy error code
    push qword 210
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr213:
    push qword 0 ;dummy error code
    push qword 211
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr214:
    push qword 0 ;dummy error code
    push qword 212
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr215:
    push qword 0 ;dummy error code
    push qword 213
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr216:
    push qword 0 ;dummy error code
    push qword 214
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr217:
    push qword 0 ;dummy error code
    push qword 215
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr218:
    push qword 0 ;dummy error code
    push qword 216
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr219:
    push qword 0 ;dummy error code
    push qword 217
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr220:
    push qword 0 ;dummy error code
    push qword 218
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr221:
    push qword 0 ;dummy error code
    push qword 219
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr222:
    push qword 0 ;dummy error code
    push qword 220
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr223:
    push qword 0 ;dummy error code
    push qword 221
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr224:
    push qword 0 ;dummy error code
    push qword 222
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr225:
    push qword 0 ;dummy error code
    push qword 223
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr226:
    push qword 0 ;dummy error code
    push qword 224
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr227:
    push qword 0 ;dummy error code
    push qword 225
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr228:
    push qword 0 ;dummy error code
    push qword 226
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr229:
    push qword 0 ;dummy error code
    push qword 227
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr230:
    push qword 0 ;dummy error code
    push qword 228
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr231:
    push qword 0 ;dummy error code
    push qword 229
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr232:
    push qword 0 ;dummy error code
    push qword 230
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr233:
    push qword 0 ;dummy error code
    push qword 231
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr234:
    push qword 0 ;dummy error code
    push qword 232
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr235:
    push qword 0 ;dummy error code
    push qword 233
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr236:
    push qword 0 ;dummy error code
    push qword 234
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr237:
    push qword 0 ;dummy error code
    push qword 235
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr238:
    push qword 0 ;dummy error code
    push qword 236
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr239:
    push qword 0 ;dummy error code
    push qword 237
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr240:
    push qword 0 ;dummy error code
    push qword 238
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr241:
    push qword 0 ;dummy error code
    push qword 239
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr242:
    push qword 0 ;dummy error code
    push qword 240
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr243:
    push qword 0 ;dummy error code
    push qword 241
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr244:
    push qword 0 ;dummy error code
    push qword 242
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr245:
    push qword 0 ;dummy error code
    push qword 243
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr246:
    push qword 0 ;dummy error code
    push qword 244
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr247:
    push qword 0 ;dummy error code
    push qword 245
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr248:
    push qword 0 ;dummy error code
    push qword 246
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr249:
    push qword 0 ;dummy error code
    push qword 247
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr250:
    push qword 0 ;dummy error code
    push qword 248
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr251:
    push qword 0 ;dummy error code
    push qword 249
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr252:
    push qword 0 ;dummy error code
    push qword 250
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr253:
    push qword 0 ;dummy error code
    push qword 251
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr254:
    push qword 0 ;dummy error code
    push qword 252
    push rax
    mov rax, interrupt_handler
    jmp push_registers_and_call_handler

isr255:
    push qword 0 ;dummy error code
    push qword 253
    push rax
    mov rax, interrupt_handler
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
