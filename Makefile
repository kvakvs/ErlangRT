.PHONY: build build-rel asm asm-rel run run-bt test

# Compile and run
run:
	cargo run

# Run with backtrace on
run-bt:
	RUST_BACKTRACE=1 cargo run


build:
	cargo build

build-rel:
	cargo build --release

asm:
	cargo rustc -- --emit asm

asm-rel:
	cargo rustc --release -- --emit asm

doc:
	cargo rustdoc -- \
	    --no-defaults \
	    --passes strip-hidden \
	    --passes collapse-docs \
	    --passes unindent-comments \
	    --passes strip-priv-imports

test:
	cargo test -- --nocapture

