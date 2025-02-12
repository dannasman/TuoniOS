kernel		:= tuoni
target		:= aarch64-unknown-none
qemu		:= qemu-system-aarch64
machine		:= virt

ifeq ($(machine), raspi4b)
qemuflags	:= -machine raspi4b -nographic -s
rustflags	:= -C link-arg=-Tsrc/bsp/raspi4b/linker.ld -C target-cpu=cortex-a72
features	:= --features=raspi4b
else
qemuflags	:= -machine virt -m 2G -cpu cortex-a72 -nographic -s
rustflags	:= -C link-arg=-Tsrc/bsp/qemu/linker.ld -C target-cpu=cortex-a72
endif

.PHONY: all clean run kernel

all: kernel-release

run: kernel-release
	cd tuoni && $(qemu) $(qemuflags) -kernel target/$(target)/release/$(kernel)

debug: kernel-debug
	cd tuoni && $(qemu) $(qemuflags) -S -kernel target/$(target)/debug/$(kernel)

kernel-release: 
	cd tuoni && RUSTFLAGS="$(rustflags)" cargo build --release $(features)

kernel-debug:
	cd tuoni && RUSTFLAGS="$(rustflags)" cargo build $(features)

kernel-objs:
	cd tuoni && RUSTFLAGS="$(rustflags)" cargo rustc $(features) -- --emit=obj

kernel-asm:
	cd tuoni && RUSTFLAGS="$(rustflags)" cargo rustc $(features) --release -- --emit=asm

kernel-type-sizes: clean
	cd tuoni && RUSTFLAGS="$(rustflags)" cargo rustc $(features) -- -Zprint-type-sizes

kernel-image: kernel-release
	cd tuoni && rust-objcopy target/$(target)/release/$(kernel) -O binary kernel8.img

clean:
	cd tuoni && cargo clean && cargo fmt
