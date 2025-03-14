arch ?= x86
kernel := build/kernel-$(arch).bin
iso := build/aquilion-$(arch).iso
target ?= $(arch)
rust_lib := target/$(target)/debug/libkernel.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso cargo-build

all: $(kernel)

clean:
	@rm -rf build/kernel-x86.bin

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): cargo-build $(assembly_object_files)
	@mkdir -p $(shell dirname $@)
	@echo "Linking with $(rust_lib)"
	@ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_lib)

kernel:
	@cargo build -p kernel
	@echo "Rust build complete"

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@