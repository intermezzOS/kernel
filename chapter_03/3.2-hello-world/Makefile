build: multiboot_header.asm boot.asm linker.ld
	nasm -f elf64 multiboot_header.asm
	nasm -f elf64 boot.asm
	ld --nmagic --output=kernel.bin --script=linker.ld multiboot_header.o boot.o
