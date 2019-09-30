# beeOS

## Why?

Why not?

## What does it do?

Well, have you ever wanted some computer which just prints the bee movie script?
Now you have the possibility to.

## How do I compile it?

Glad you asked! First, you need some tools:

 - nasm
 - nightly rust (the one as of September 30th 2019 works fine)
 - xargo (cargo install xargo)
 - make
 - grub-mkrescue (bundled with GRUB2)
 - GNU/mtools
 - probably an x86_64 system to compile on, that's what I used
 - either xorriso or libiso to use with grub-mkrescue

How:

```bash
$ cd asm
$ make build
# want to run it with qemu-system-x86_64?
$ make run
```
