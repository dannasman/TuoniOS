kernel			:= tuoni
target			:= aarch64-unknown-none
qemu			:= qemu-system-aarch64
machine			:= virt
kernel_manifest := kernel/Cargo.toml

ifeq ($(machine), raspi4b)
qemuflags	:= -machine raspi4b -m 2G -nographic -s
rustflags	:= -C link-arg=-Tkernel/src/bsp/raspi4b/linker.ld -C target-cpu=cortex-a72
features	:= --features=raspi4b
else
qemuflags	:= -machine virt -m 2G -cpu cortex-a72 -nographic -s
rustflags	:= -C link-arg=-Tkernel/src/bsp/qemu/linker.ld -C target-cpu=cortex-a72
endif

.PHONY: all clean run kernel

all: kernel-release

run: kernel-release
	$(qemu) $(qemuflags) -kernel target/$(target)/release/$(kernel)

debug: kernel-debug
	$(qemu) $(qemuflags) -S -kernel target/$(target)/debug/$(kernel)

kernel-release: 
	RUSTFLAGS="$(rustflags)" cargo build --manifest-path $(kernel_manifest) --release $(features)

kernel-debug:
	RUSTFLAGS="$(rustflags)" cargo build --manifest-path $(kernel_manifest) $(features)

kernel-objs:
	RUSTFLAGS="$(rustflags)" cargo rustc --manifest-path $(kernel_manifest) $(features) -- --emit=obj

kernel-asm:
	RUSTFLAGS="$(rustflags)" cargo rustc --manifest-path $(kernel_manifest) $(features) --release -- --emit=asm

kernel-type-sizes: clean
	RUSTFLAGS="$(rustflags)" cargo rustc --manifest-path $(kernel_manifest) $(features) -- -Zprint-type-sizes

kernel-image: kernel-release
	rust-objcopy --strip-all -O binary target/$(target)/release/$(kernel) kernel8.img

clean:
	cargo clean && cargo fmt
