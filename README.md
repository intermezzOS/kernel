# Nucleus

This is a small hobby OS, written in Rust. It is not particularly serious. I’m
not yet at the point where I have to actually worry about design decisions, so
we’ll see what it ends up like.

http://jvns.ca/blog/2014/03/12/the-rust-os-story/ is a wonderful example of the
spirit of this project. The Rust bits are outdated, the “Let’s learn things!
Learning is great!” attitude is very much not.

![screenshot][screenshot.png]

Resources:

* http://wiki.osdev.org/
* http://pages.cs.wisc.edu/~remzi/OSTEP/
* http://blog.phil-opp.com/rust-os/multiboot-kernel.html
* http://www.randomhacks.net/bare-metal-rust/

## Building

First of all, at the moment, this only works on `x86_64` Linux. I need to set
up a proper cross compiler. It’s early stages, and I’m lazy. :wink:

Make sure you have a nightly Rust. Any recent nightly should do. Other requirements:

* `nasm`
* `ld`
* `grub-mkrescue`
* `qemu-system-x86_64`

If you have all that, type:

```bash
$ make run
```

This will:

* Build the Rust code with Cargo
* Compile the bit of assembly needed to boot
* Link it all together
* Build an ISO out of it all
* Run that ISO in qemu
