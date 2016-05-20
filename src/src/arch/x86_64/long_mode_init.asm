global long_mode_start

section .text
bits 64
long_mode_start:

    ; set up SSE
    mov rax, cr0
    and ax, 0xFFFB
    or ax, 0x2
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9
    mov cr4, rax

    ; remap PIC
    in al, 0x21
    mov cl, al
    in al, 0xA1
    mov ch, al

    mov al, 0x11
    out 0x20, al
    out 0xA0, al

    mov al, 0x20
    out 0x21, al
    mov al, 0x28
    out 0xA1, al

    mov al, 4
    out 0x21, al
    mov al, 2
    out 0xA1, al

    mov al, 0x1
    out 0x21, al
    out 0xA1, al

    mov al, cl
    out 0x21, al
    mov al, ch
    out 0xA1, al

    extern kmain
    call kmain

    hlt
