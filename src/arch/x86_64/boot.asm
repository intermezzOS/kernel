global start

section .text
bits 32
start:
    ; print `OK` to screen
    ; mov dword [0xb8000], 0x2f4b2f4f
    mov dword [0xb8000], 0x914F914c
    mov dword [0xb8004], 0x0000914c
    hlt

