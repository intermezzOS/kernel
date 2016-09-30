ENTRY(start)

MAGIC_NUMBER = 0xe85250d6;

SECTIONS {
  . = 1M;

  .boot :
  {
    header_start = .;

    /* Magic number */
    LONG(MAGIC_NUMBER)
    /* Architecture 0: Protected mode i386 */
    LONG(0)
    /* Header length */
    LONG(header_end - header_start)
    /* Checksum */
    LONG(0x100000000 - (MAGIC_NUMBER + (header_end - header_start)))
    /* Required end tag */
    /* Type */
    SHORT(0)
    /* Flags */
    SHORT(0)
    /* Size */
    LONG(8)

    header_end = .;
 }

  .rodata :
  {
    *(.rodata .rodata.*)
    . = ALIGN(4K);
  }

  .text :
  {
    *(.text .text.*)
    . = ALIGN(4K);
  }

  .data :
  {
    *(.data .data.*)
    . = ALIGN(4K);
  }

  .bss :
  {
    *(.bss .bss.*)
    . = ALIGN(4K);
  }

  /DISCARD/ :
  {
    *(.note.gnu.*)
  }
}
