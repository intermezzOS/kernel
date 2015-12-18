
ENTRY(start)

kernel_stack_size = 1M;

SECTIONS {

    /* somehow needed for grub to recognize the multiboot header */
    . = 2M + SIZEOF_HEADERS;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot_header))

        /* ensure that the address (tss-0x200000) fits in 16bit */
        *(.tss)
        /* ensure that the address (interrupt_handlers-0x200000) fits in 16bit*/
        *(.interrupt_handlers)
    }

    .text :
    {
      *(.text .text.*)
    }

    .rodata :
    {
      *(.rodata .rodata.*)
    }

    .data.rel.ro :
    {
      *(.data.rel.ro .data.rel.ro.*)
    }

    .data :
    {
      *(.data .data.*)
    }

    kernel_stack_bottom = .;
    . += kernel_stack_size;
    . = ALIGN(16);
    kernel_stack_top = .;

}

