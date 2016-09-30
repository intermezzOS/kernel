.intel_syntax noprefix
.extern kmain
.globl start

.section .boot, "ax"
.code32  # Tell assembler to generate 32-bit code now.
start:

    # Point the first entry of the level 4 page table to the first entry in the
    # p3 table
    mov eax, p3_table
    or eax, 0b11 ; 
    mov dword [p4_table + 0], eax

    # Point the first entry of the level 3 page table to the first entry in the
    # p2 table
    mov eax, p2_table
    or eax, 0b11
    mov dword [p3_table + 0], eax

    # point each page table level two entry to a page
    mov ecx, 0         # counter variable
.map_p2_table:
    mov eax, 0x200000  # 2MiB
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    # move page table address to cr3
    mov eax, p4_table
    mov cr3, eax

    # enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    # set the long mode bit
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    # enable paging
    mov eax, cr0
    or eax, 1 << 31
    or eax, 1 << 16
    mov cr0, eax

    lgdt [gdt_pointer]

    # update selectors
    mov ax, 16
    mov ss, ax
    mov ds, ax
    mov es, ax

    ljmp 8, kmain

    # shouldn't ever happen
    hlt

.section .bss
.align 4096
.lcomm p4_table, 4096
.lcomm p3_table, 4096
.lcomm p2_table, 4096

.section .rodata
gdt:
.quad 0
.quad 0x0020980000000000
.quad 0x0000920000000000

gdt_pointer:
.word 23
.quad gdt
