kernel    := tuoni
target    := aarch64-unknown-none
qemu      := qemu-system-aarch64
qemuflags := -machine virt -m 2G -cpu cortex-a72 -nographic -s

.PHONY: all clean run kernel

all: kernel-release

clean:
	cd tuoni && cargo clean && cargo fmt

run: kernel-release
	cd tuoni && $(qemu) $(qemuflags) -kernel target/$(target)/release/$(kernel)

debug: kernel-debug
	cd tuoni && $(qemu) $(qemuflags) -S -kernel target/$(target)/debug/$(kernel)

kernel-release: 
	cd tuoni && cargo build --release

kernel-debug:
	cd tuoni && cargo build

type-sizes:
	cd tuoni && cargo rustc -- -Zprint-type-sizes
