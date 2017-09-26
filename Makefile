.PHONY: build asm asm-rel

build:
	cargo build

asm:
	cargo rustc -- --emit asm

asm-rel:
	cargo rustc --release -- --emit asm
