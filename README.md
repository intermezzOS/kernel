# intermezzOS: kernel

[![Build Status](https://travis-ci.org/intermezzOS/kernel.svg?branch=master)](https://travis-ci.org/intermezzOS/kernel)

intermezzOS is a hobby operating system. This repository is for its kernel.
See [the website](http://intermezzos.github.io/) for more.

Also, feel free to join us at `#intermezzOS` on Freenode’s IRC network, if you
want to chat.

## Building

To build intermezzOS, first you'll need the source:

```bash
$ git clone https://github.com/intermezzOS/kernel
$ cd kernel
```

Next, make sure you're using nightly Rust, and that you have the source
code installed as well. The official way to do this is with the `rustup`
tool [from Rust's website](http://rust-lang.org/install.html).

```bash
$ rustup override add nightly
$ rustup component add rust-src
$ rustup update nightly
```

You'll also need [xargo](https://github.com/japaric/xargo). To get it:

 ```bash
 $ cargo install xargo
 ```
 
You'll also need some other things installed on your system; how you get
them depends on what system you're running. Consult their documentation to
figure out exactly how to install:

* `nasm`
* `ld`
* `grub-mkrescue`: you may also need to install `xorriso`
* `qemu-system-x86_64`

If you're on Bash On Windows, you'll need the `grub-pc-bin` package, as well as
an X server of some kind, like
[xming](https://sourceforge.net/projects/xming/). You'll also need to run
this:

```bash
$ export DISPLAY=:0
```

You can put it in your `~/.bashrc` file to have it automatically work on each
session.

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

* Build libcore for our x86_64-unknown-intermezzos-gnu target with Xargo
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
