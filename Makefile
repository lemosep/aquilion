ROOTDIR := $(PWD)
BOOT_DIR := $(ROOTDIR)/bootloader

boot-build:
	@cd $(BOOT_DIR) && make build

boot-run:
	@cd $(BOOT_DIR) && make run

boot-clean:
	@cd $(BOOT_DIR) && make clean
		