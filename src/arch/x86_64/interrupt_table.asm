BITS 64

global setup_interrupt_table


section .text
setup_interrupt_table:
    lidt [idt_pointer]
    ret


section .data

%macro IDT_ENTRY 1
    extern interrupt_handler_%1
    ;TODO: remove the word data exceeds bounds warning
    DW (interrupt_handler_%1 - 0x200000)  ;offset_low
    DW 0x8              ;text segment

    %ifdef H%1_IST
        DB H%1_IST      ;interrupt stack table index
    %elif
        DB 0            ;no stack switch (if cpl doesn't change)
    %endif

    DB 0x8e             ;type_addr: present+interrupt_gate
    DW 0x20             ;offset_middle
    DQ 0                ;offset_high and zero2
%endmacro

idt:
    %assign i 0
    %rep 256
        IDT_ENTRY i
    %assign i i+1
    %endrep

idt_pointer:
    dw 4095   ; limit
    dq idt
