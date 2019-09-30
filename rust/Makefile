arch ?= x86_64
output := target/$(arch)-beeos/release/libbeeos.a

.PHONY: build
build: $(output)

.PHONY: clean
clean:
	cargo clean

$(output):
	RUST_TARGET_PATH=$(shell pwd) xargo build --release --target $(arch)-beeos
