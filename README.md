# intermezzOS: kernel

intermezzOS is a hobby operating system. This repository is for its kernel.
See [the website](http://intermezzos.github.io/) for more.

Also, feel free to join us at `#intermezzOS` on Freenode’s IRC network, if you
want to chat.

We’re currently in the process of re-building from scratch, to properly
document things as we go along. Check the `original_backup` branch for the code
we had before this redux.

## Requirements

Right now, things are only guaranteed to work on 64-bit Linux.

* `nasm`
* `ld`
* `grub-mkrescue`, possibly with `xorriso`, depending on your package manager
* `qemu`, specifically `qemu-system-x86_64`

## CURRENT STATUS

Right now, if you run `make` you’ll
see this:

![screenshot](screenshot.png)

Okay indeed!
