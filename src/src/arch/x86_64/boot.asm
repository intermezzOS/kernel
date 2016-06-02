global start

section .boot
bits 32
start:

    ; Point the first entry of the level 4 page table to the first entry in the
    ; p3 table
    mov eax, p3_table
    or eax, 0b11 ; 
    mov dword [p4_table + 0], eax

    ; Point the first entry of the level 3 page table to the first entry in the
    ; p2 table
    mov eax, p2_table
    or eax, 0b11
    mov dword [p3_table + 0], eax

    ; point each page table level two entry to a page
    mov ecx, 0         ; counter variable
.map_p2_table:
    mov eax, 0x200000  ; 2MiB
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ; move page table address to cr3
    mov eax, p4_table
    mov cr3, eax

    ; enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31
    or eax, 1 << 16
    mov cr0, eax

    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    ; long jump to long_mode_start setting `cs` register to `gdt64.code`
    jmp gdt64.code:long_mode_start

    ; shouldn't ever happen
    hlt

section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 64
stack_top:

section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53) ; code segment
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) ; data segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

section .text
bits 64
long_mode_start:

    call setup_SSE
    call remap_PIC

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

