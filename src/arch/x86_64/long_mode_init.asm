global long_mode_start

section .text
bits 64
long_mode_start:
    ; print `LOL` to screen
    mov rax, 0x914c914F914c
    mov qword [0xb8000], rax
    hlt

