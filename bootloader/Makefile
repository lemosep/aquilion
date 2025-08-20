BINDIR = bin

build:
	@mkdir -p $(BINDIR)
	@nasm boot.asm -f bin -o $(BINDIR)/boot.bin

run: build
	@qemu-system-x86_64 -drive file="$(BINDIR)/boot.bin",format=raw,if=floppy

clean:
	@rm -rf $(BINDIR)