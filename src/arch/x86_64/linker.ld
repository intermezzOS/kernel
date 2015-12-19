ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot_header))

        /* ensure that the address (tss-0x100000) fits in 16bit */
        *(.tss)
        /* ensure that the address (interrupt_handlers-0x100000) fits in 16bit*/
        *(.interrupt_handlers)
    }

    .text :
    {
        *(.boot)
        *(.text)
    }
}
