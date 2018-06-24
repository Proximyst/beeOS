# beeOS

## Why?

Why not?

## What does it do?

It prints the Bee Movie script. Every line of it. Forever.

## How do I use it?

### Precompiled

Requirements:

 - An ISO or BIN from the releases page
 - An x86_64 compatible machine or VM
 - A flash stick (if machine)

Now head over to either the `Install` or the `Qemu` category.

### Compile

Requirements:

 - Git
 - Cargo
 - Rust with nightly toolchain
 - (rust-osdev/bootimage)[https://github.com/rust-osdev/bootimage]

How:

 1. Clone the repository using git.
 1. Run `bootimage build --release`. (If you want to use Qemu, head over to the `Qemu` category now - you can alternatively use `bootimage run`).
 1. Run `mv target/x86_64-minimal_os/release/bootimage-minimal_os.bin iso/boot/kernel.bin`.
 1. Run `grub-mkrescue -o beeOS.iso iso`

### Qemu

Requirements:

 - Qemu with support for x86_64
 - A BIN file of the OS

How:

 1. Run `qemu-system-x86_64 -drive format=raw,file=WRITE_THE_ISO_FILENAME_HERE`

### Install

Requirements:

 - Linux's `dd`
 - An ISO file of the OS

How:

 1. Run `dd if=WRITE_THE_ISO_FILENAME_HERE of=/dev/sd# bs=4M`
