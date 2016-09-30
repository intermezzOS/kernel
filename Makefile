cargo: 
	xargo build --release --target x86_64-unknown-intermezzos-gnu

# cargo test fails for some reason, not sure why yet
test:
	cd console && cargo test

iso: cargo grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp grub.cfg target/isofiles/boot/grub
	cp target/x86_64-unknown-intermezzos-gnu/release/intermezzos target/isofiles/boot/
	grub-mkrescue -o target/os.iso target/isofiles

run: iso
	qemu-system-x86_64 -cdrom target/os.iso

debug: iso
	qemu-system-x86_64 -s -S -cdrom target/os.iso

gdb: iso
	gdb target/x86_64-unknown-intermezzos-gnu/release/intermezzos -ex "target remote :1234"

asm:
	gcc -m64 -c src/asm/boot.S -o gccboot.o
	nasm -felf64 boot.asm -o nasmboot.o
