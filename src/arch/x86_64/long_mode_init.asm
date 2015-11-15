global long_mode_start

section .text
bits 64
long_mode_start:

    extern kmain
    call kmain

    hlt

