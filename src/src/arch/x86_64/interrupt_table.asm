BITS 64

global load_idt
global idt
extern IDT_POINTER

section .text
load_idt:
    lidt [IDT_POINTER]
    ret

section .data

%macro IDT_ENTRY 1
    extern isr%1
    ;TODO: remove the word data exceeds bounds warning
    DW (isr%1 - 0x100000)  ;offset_low
    DW 0x8              ;text segment

    %ifdef H%1_IST
        DB H%1_IST      ;interrupt stack table index
    %elif
        DB 0            ;no stack switch (if cpl doesn't change)
    %endif

    DB 0x8e             ;type_addr: present+interrupt_gate
    DW 0x10             ;offset_middle
    DQ 0                ;offset_high and zero2
%endmacro

idt:
    %assign i 0
    %rep 256
    %if i != 33
        IDT_ENTRY i
    %else
        IDT_ENTRY 1
    %endif
    %assign i i+1
    %endrep
