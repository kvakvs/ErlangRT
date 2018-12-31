.PHONY: build emu_codegen
build: emu_codegen
	cargo build

emu_codegen:
	cd emulator && $(MAKE) codegen
