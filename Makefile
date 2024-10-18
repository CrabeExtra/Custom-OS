# I just know I'm going to not know how to re-write this.

# Define variables for paths and tools
RUST_TARGET := x86_64-unknown-none
BUILD_DIR := build
ISO_DIR := dist/x86_64

CARGO_BUILD := cargo build --release --target=$(RUST_TARGET)

X86_64_ASM_SOURCE_FILES := $(shell find src/impl/x86_64 -name *.asm)
X86_64_ASM_OBJECT_FILES := $(patsubst src/impl/x86_64/%.asm, build/x86_64/%.o, $(X86_64_ASM_SOURCE_FILES))

.PHONY: build-x86_64

build-x86_64: $(X86_64_ASM_OBJECT_FILES)
    # Step 1: Compile the Rust project
    $(CARGO_BUILD)

    # Step 2: Compile assembly files with NASM
    @mkdir -p $(BUILD_DIR)
    nasm -f elf64 src/impl/x86_64/boot.asm -o $(BUILD_DIR)/boot.o

    # Step 3: Link the kernel including Rust's binary
    ld -n -o $(ISO_DIR)/kernel.bin -T targets/x86_64/linker.ld $(BUILD_DIR)/boot.o target/$(RUST_TARGET)/release/libyourkernel.a

    # Step 4: Prepare the ISO directory
    @mkdir -p targets/x86_64/iso/boot/grub

    # Step 5: Copy files to the ISO structure
    @cp $(ISO_DIR)/kernel.bin targets/x86_64/iso/boot/kernel.bin
    @cp grub.cfg targets/x86_64/iso/boot/grub/grub.cfg

    # Step 6: Create the ISO using GRUB
    grub-mkrescue -o $(ISO_DIR)/youros.iso targets/x86_64/iso

# Rule to compile assembly files
$(BUILD_DIR)/x86_64/%.o: src/impl/x86_64/%.asm
    @mkdir -p $(dir $@)
    nasm -f elf64 $< -o $@

# Clean up build artifacts
clean:
    rm -rf $(BUILD_DIR) $(ISO_DIR)