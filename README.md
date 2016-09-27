# intermezzOS: kernel

[![Build Status](https://travis-ci.org/intermezzOS/kernel.svg?branch=master)](https://travis-ci.org/intermezzOS/kernel)

intermezzOS is a hobby operating system. This repository is for its kernel.
See [the website](http://intermezzos.github.io/) for more.

Also, feel free to join us at `#intermezzOS` on Freenode’s IRC network, if you
want to chat.

## Building

Make sure you have a nightly Rust installed. Something like this:

```bash
$ rustc --version
rustc 1.13.0-nightly (267cde259 2016-05-25)
```

If you don’t have Rust installed (and maybe even if you've installed Rust some
other way), you should use [rustup](https://rustup.rs/) to get it; it makes
using multiple Rust versions very easy.

Then get this stuff:

* `nasm`
* `ld`
* `grub-mkrescue`: you may also need to install `xorriso`
* `qemu-system-x86_64`

If you're on OS X, you might want to use [this
script](http://intermezzos.github.io/book/appendix/osx-install.html) to get
them. You'll also need to set this in your `~/.cargo/config`:

```toml
[target.x86_64-unknown-intermezzos-gnu]
linker = "/Users/yourusername/opt/bin/x86_64-pc-elf-gcc"
```

Where `yourusername` is your username.

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

## License

This project is dual licensed under Apache2/MIT. See the two `LICENSE-*` files
for details.

## Code of conduct

The intermezzOS project has a [Code of
Conduct](http://intermezzos.github.io/code-of-conduct.html), please observe it.
