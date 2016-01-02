default: run

.PHONY: clean

multiboot_header.o: multiboot_header.asm
	nasm -f elf64 multiboot_header.asm

boot.o: boot.asm
	nasm -f elf64 boot.asm

kernel.bin: multiboot_header.o boot.o linker.ld
	ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o

isofiles: kernel.bin grub.cfg
	mkdir -p isofiles/boot/grub
	cp grub.cfg isofiles/boot/grub
	cp kernel.bin isofiles/boot/

os.iso: isofiles
	grub-mkrescue -o os.iso isofiles

build: os.iso

run: os.iso
	qemu-system-x86_64 -cdrom os.iso

clean: 
	rm -f multiboot_header.o
	rm -f boot.o
	rm -f kernel.bin
	rm -rf isofiles
	rm -f os.iso
