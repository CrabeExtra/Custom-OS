# I just know I'm going to not know how to re-write this.

# Define variables for paths and tools
RUST_TARGET := x86_64-unknown-none
BUILD_DIR := build
ISO_DIR := dist/x86_64

CARGO_BUILD := cargo build --release --target=$(RUST_TARGET)

X86_64_ASM_SOURCE_FILES := $(shell find src/impl/x86_64 -name *.asm)
X86_64_ASM_OBJECT_FILES := $(patsubst src/impl/x86_64/%.asm, build/x86_64/%.o, $(X86_64_ASM_SOURCE_FILES))

# Rule to compile assembly files
$(BUILD_DIR)/x86_64/%.o: src/impl/x86_64/%.asm
	@mkdir -p $(dir $@)
	nasm -f elf64 $< -o $@

.PHONY: build-x86_64

build-x86_64: $(X86_64_ASM_OBJECT_FILES)
	# Step 1: Compile the Rust project
	$(CARGO_BUILD)

	# Step 2: Compile assembly files with NASM
	ld -n -o $(ISO_DIR)/kernel.bin -T targets/x86_64/linker.ld $(X86_64_ASM_OBJECT_FILES) target/$(RUST_TARGET)/release/libcustom_os_rust.a

	# Step 3: Link the kernel including Rust's binary
	ld -n -o $(ISO_DIR)/kernel.bin -T targets/x86_64/linker.ld $(BUILD_DIR)/x86_64/boot/main.o target/$(RUST_TARGET)/release/libcustom_os_rust.a

	# Step 4: Prepare the ISO directory
	@mkdir -p targets/x86_64/iso/boot/grub

	# Step 5: Copy files to the ISO structure
	@cp $(ISO_DIR)/kernel.bin targets/x86_64/iso/boot/kernel.bin
	@cp grub.cfg targets/x86_64/iso/boot/grub/grub.cfg

	# Step 6: Create the ISO using GRUB
	grub-mkrescue -o $(ISO_DIR)/kernel.iso targets/x86_64/iso

# Clean up build artifacts
clean:
	$(BUILD_DIR) $(ISO_DIR)
	