arch ?= x86_64
rust_target ?= $(arch)-beeos
build_dir := build/$(arch)
src_dir := src/arch/$(arch)
grub_cfg := src/iso/boot/grub/grub.cfg
asm_src := $(wildcard $(src_dir)/*.asm)
asm_obj := $(patsubst $(src_dir)/%.asm, $(build_dir)/%.o, $(asm_src))
kernel := $(build_dir)/iso/boot/kernel.bin
linker := $(src_dir)/link.ld
rust_interface := ../rust/beeos/target/$(arch)-beeos/release/libbeeos.a
iso := $(build_dir)/beeOS-$(arch).iso

.PHONY: build
build: $(build_dir) $(asm_obj) $(kernel)
	mkdir -p $(build_dir)/iso/boot/grub
	cp $(grub_cfg) $(build_dir)/iso/boot/grub

.PHONY: clean
clean:
	rm -rf build

$(build_dir):
	mkdir -p $(build_dir)
	cp -r src/iso $(build_dir)/iso

$(build_dir)/%.o: $(src_dir)/%.asm
	mkdir -p $(build_dir)
	nasm -felf64 $< -o $@

$(rust_interface):

.PHONY: kernel
kernel: $(kernel)

$(kernel): $(rust_interface) $(build_dir) $(asm_obj) $(linker) $(wildcard ../rust/beeos/src/*.rs)
	cd ../rust/beeos && RUST_TARGET_PATH=./ xargo build --release --target x86_64-beeos
	$(LD) -n -o $(kernel) -T $(linker) $(asm_obj) $(rust_interface)

.PHONY: iso
iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	grub-mkrescue -o $(iso) $(build_dir)/iso

.PHONY: run
run: iso
	qemu-system-x86_64 -cdrom $(iso)
