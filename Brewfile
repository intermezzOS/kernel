# for assembling x86
brew 'nasm'

# for making a bootable ISO
brew 'xorriso'

# for running the OS
brew 'qemu'

# x86_64-pc-elf cross-compile toolchain
tap 'hawkw/x86_64-pc-elf'
brew 'x86_64-pc-elf-gcc'

# GRUB
tap 'hawkw/grub'
brew 'grub', args: ['with-x86_64-pc-elf', 'HEAD']
