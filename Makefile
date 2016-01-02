default: run

.PHONY: clean

multiboot_header.o: multiboot_header.asm
	nasm -f elf64 multiboot_header.asm

boot.o: boot.asm
	nasm -f elf64 boot.asm

kernel.bin: multiboot_header.o boot.o linker.ld
	ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o

os.iso: kernel.bin
	cp kernel.bin isofiles/boot/
	grub-mkrescue -o os.iso isofiles

build: os.iso

run: os.iso
	qemu-system-x86_64 -cdrom os.iso

clean: 
	rm -f multiboot_header.o
	rm -f boot.o
	rm -f kernel.bin
	rm -f isofiles/boot/kernel.bin
	rm -f os.iso
