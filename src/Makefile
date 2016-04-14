arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
    build/arch/$(arch)/%.o, $(assembly_source_files))

target ?= $(arch)-unknown-intermezzos-gnu
rust_os := target/$(target)/debug/libintermezzos.a

.PHONY: all clean run iso kernel

all: $(iso)

clean:
	rm -rf build
	rm -rf target

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	rm -r build/isofiles

kernel: $(kernel)

$(kernel): cargo $(rust_os) $(assembly_object_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

cargo:
	cargo build --target $(target)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -g -felf64 $< -o $@

docs: book

book: docs/book/
	cd docs && mdbook build
