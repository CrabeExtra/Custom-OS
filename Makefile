RUST_TARGET := x86_64-unknown-none
BUILD_DIR := build
ISO_DIR := dist/x86_64

# Find all Rust source files
RS_SOURCE_FILES := $(shell find src -name '*.rs')
RS_OBJECT_FILES := $(patsubst src/%.rs, $(BUILD_DIR)/%.o, $(RS_SOURCE_FILES))

# Find all assembly source files
X86_64_ASM_SOURCE_FILES := $(shell find src/impl/x86_64 -name '*.asm')
X86_64_ASM_OBJECT_FILES := $(patsubst src/impl/x86_64/%.asm, $(BUILD_DIR)/x86_64/%.o, $(X86_64_ASM_SOURCE_FILES))

# Compile Assembly
$(BUILD_DIR)/x86_64/%.o: src/impl/x86_64/%.asm
	@mkdir -p $(dir $@)
	nasm -f elf64 $< -o $@

# Compile Rust
$(BUILD_DIR)/%.o: src/%.rs
	@mkdir -p $(dir $@)
	rustc --emit=obj -o $@ --crate-type lib --target=$(RUST_TARGET) $<

# Combine all object files
ALL_OBJECT_FILES := $(X86_64_ASM_OBJECT_FILES) $(RS_OBJECT_FILES)

.PHONY: build-x86_64 clean

# Build and link the kernel
build-x86_64: $(ALL_OBJECT_FILES)
	@mkdir -p $(ISO_DIR)
	x86_64-elf-ld -n -o $(ISO_DIR)/kernel.bin -T targets/x86_64/linker.ld $(ALL_OBJECT_FILES)
	@mkdir -p targets/x86_64/iso/boot/grub
	@cp $(ISO_DIR)/kernel.bin targets/x86_64/iso/boot/kernel.bin
	grub-mkrescue /usr/lib/grub/i386-pc -o $(ISO_DIR)/kernel.iso targets/x86_64/iso
