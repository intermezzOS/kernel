extern setup_interrupt_table

global long_mode_start

section .text
bits 64
long_mode_start:

    call setup_SSE
    call remap_PIC
    call setup_interrupt_table

    extern kmain
    call kmain

    hlt

setup_SSE:
    mov rax, cr0
    and ax, 0xFFFB      ; clear coprocessor emulation CR0.EM
    or ax, 0x2          ; set coprocessor monitoring  CR0.MP
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9       ; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
    mov cr4, rax

    ret

remap_PIC:
    in al, 0x21                   ; save pic1 mask
    mov cl, al
    in al, 0xA1                   ; save pic2 mask
    mov ch, al

    mov al, 0x11
    out 0x20, al                ; send initialize command to pic1
    out 0xA0, al                ; send initialize command to pic2

    mov al, 0x20
    out 0x21, al                ; set vector offset of pic1 to 0x20
    mov al, 0x28
    out 0xA1, al                ; set vector offset of pic2 to 0x28

    mov al, 4
    out 0x21, al                   ; tell pic1 that there is a slave PIC at IRQ2 (0000 0100)
    mov al, 2
    out 0xA1, al                   ; tell pic2 its cascade identity (0000 0010)

    mov al, 0x1
    out 0x21, al                 ; 8086 mode for pic1
    out 0xA1, al                 ; 8086 mode for pic2

    mov al, cl
    out 0x21, al                  ; restore pic1 mask
    mov al, ch
    out 0xA1, al                  ; restore pic2 mask

    ret

