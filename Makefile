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

chainloader_manifest	:= chainloader/Cargo.toml
chainloader_rustflags	:=	-C link-arg=-Tchainloader/src/linker.ld -C target-cpu=cortex-a72
chainloader_qemuflags	:= -machine raspi4b -m 2G -nographic -s

.PHONY: all clean run kernel chainloader

all: kernel-release

run: kernel-release
	$(qemu) $(qemuflags) -kernel target/$(target)/release/$(kernel)

debug: kernel-debug
	$(qemu) $(qemuflags) -S -kernel target/$(target)/debug/$(kernel)

define TEST_RUNNER
#!/usr/bin/env bash
	cd $(shell pwd)
	TEST_ELF=$$(echo $$1 | sed -e 's/.*target/target/g')
	$(qemu) $(qemuflags) -kernel $$TEST_ELF
endef

export TEST_RUNNER

define test_prepare
	@mkdir -p target
	@echo "$$TEST_RUNNER" > target/test_runner.sh
	@chmod +x target/test_runner.sh
endef

test_unit:
	$(call test_prepare)
	RUSTFLAGS="$(rustflags)" cargo test --manifest-path $(kernel_manifest) $(features) --release --lib

test_integration:
	$(call test_prepare)
	RUSTFLAGS="$(rustflags)" cargo test --manifest-path $(kernel_manifest) $(features) --release --test '*'

test: test_unit test_integration

run-chainloader: chainloader-release
	$(qemu) $(chainloader_qemuflags) -kernel target/$(target)/release/chainloader

debug-chainloader: chainloader-debug
	$(qemu) $(chainloader_qemuflags) -S -kernel target/$(target)/debug/chainloader

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

chainloader-release:
	RUSTFLAGS="$(chainloader_rustflags)" cargo rustc --manifest-path $(chainloader_manifest) --release

chainloader-debug:
	RUSTFLAGS="$(chainloader_rustflags)" cargo rustc --manifest-path $(chainloader_manifest)

chainloader-image: chainloader-release
	rust-objcopy --strip-all -O binary target/$(target)/release/chainloader kernel8.img

clean:
	cargo clean && cargo fmt
