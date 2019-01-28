.PHONY: build emu_codegen ct

build: emu_codegen
	cargo build

emu_codegen:
	cd lib-erlangrt && $(MAKE) codegen

ct: build
	mkdir tmp; cd tmp && ../target/debug/ct_run 1 2 3 -erl_args 4 5 6

