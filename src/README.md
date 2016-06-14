# IntermezzOS

This is a hobby OS, written in Rust. It is not particularly serious. I’m
not yet at the point where I have to actually worry about design decisions, so
we’ll see what it ends up like.

http://jvns.ca/blog/2014/03/12/the-rust-os-story/ is a wonderful example of the
spirit of this project. The Rust bits are outdated, the “Let’s learn things!
Learning is great!” attitude is very much not.


Resources:

* http://wiki.osdev.org/
* http://pages.cs.wisc.edu/~remzi/OSTEP/
* http://blog.phil-opp.com/rust-os/multiboot-kernel.html
* http://www.randomhacks.net/bare-metal-rust/

## Building

Make sure you have this Rust:

```bash
$ rustc --version
rustc 1.10.0-nightly (267cde259 2016-05-25)
```

It has to be this one. Exactly this one. This is the nightly for `2016-05-26`,
even though the date says `25`. They’re off sometimes.

If you don’t have Rust installed, you can do this:

```bash
$ curl -sf https://static.rust-lang.org/rustup.sh | sudo sh -- --channel=nightly --date=2016-05-26
```

If you use a tool like multirust or rustup, use that.

Then get this stuff:

* `nasm`
* `ld`
* `grub-mkrescue`: you may also need to install `xorriso`
* `qemu-system-x86_64`

After all that setup, it’s as easy as:

```bash
$ make run
```

This will:

* Build the Rust code with Cargo
* Compile the bit of assembly needed to boot
* Link it all together
* Build an ISO out of it all
* Run that ISO in qemu
