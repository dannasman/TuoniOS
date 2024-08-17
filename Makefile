kernel    := tuoni
target    := aarch64-unknown-none
qemu      := qemu-system-aarch64
qemuflags := -machine virt -m 2G -cpu cortex-a53 -nographic -s

.PHONY: all clean run kernel

all: kernel-release

clean:
	cargo clean

run: kernel-release
	$(qemu) $(qemuflags) -kernel target/$(target)/release/$(kernel)

debug: kernel-debug
	$(qemu) $(qemuflags) -S -kernel target/$(target)/debug/$(kernel)

kernel-release: 
	cargo build --release

kernel-debug:
	cargo build

type-sizes:
	cargo rustc -- -Zprint-type-sizes
