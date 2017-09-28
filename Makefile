.PHONY: build build-rel asm asm-rel run

run:
	cargo run

build:
	cargo build

build-rel:
	cargo build --release

asm:
	cargo rustc -- --emit asm

asm-rel:
	cargo rustc --release -- --emit asm
