# beeOS

## Why?

Why not?

## What does it do?

Well, have you ever wanted some computer which just prints the bee movie script?
Now you have the possibility to.

## How do I compile it?

Glad you asked! First, you need some tools:

 - nasm
 - nightly rust (the one as of September 24th 2018 works fine)
 - xargo (cargo install xargo)
 - make
 - grub-mkrescue (bundled with GRUB2)
 - probably an x86_64 system to compile on, that's what I used

How:

```bash
$ cd rust/beeos/
$ RUST_TARGET_PATH=$(pwd) xargo build --release --target x86_64-beeos
$ cd ../../asm
$ make build
# want to run it with qemu-system-x86_64?
$ make run
```
