build:
	@mkdir -p bin
	@nasm boot.asm -f bin -o ./bin/boot.bin

run:
	@qemu-system-x86_64 -fda ./bin/boot.bin