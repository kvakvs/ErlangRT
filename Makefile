.PHONY: build emu_codegen
build: emu_codegen
	cargo build

emu_codegen:
	cd emulator && $(MAKE) codegen

ct:
	mkdir tmp; cd tmp && ../target/debug/ct_run