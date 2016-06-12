build: multiboot_header.asm boot.asm linker.ld grub.cfg
	nasm -f elf64 multiboot_header.asm
	nasm -f elf64 boot.asm
	ld --nmagic --output=kernel.bin --script=linker.ld multiboot_header.o boot.o
	mkdir -p isofiles/boot/grub
	cp grub.cfg isofiles/boot/grub
	cp kernel.bin isofiles/boot/
	grub-mkrescue -o os.iso isofiles

run: build
	qemu-system-x86_64 -cdrom os.iso
