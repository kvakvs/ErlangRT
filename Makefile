HERE=$(shell pwd)

.PHONY: run run-bt
# Compile and run
run:
	cargo run

# Run with backtrace on
run-bt:
	RUST_BACKTRACE=1 cargo run

.PHONY: build build-rel asm asm-rel
build:
	cargo build

build-rel:
	cargo build --release

asm:
	cargo rustc -- --emit asm

asm-rel:
	cargo rustc --release -- --emit asm

.PHONY: doc
doc:
	cargo rustdoc -- \
	    --no-defaults \
	    --passes strip-hidden \
	    --passes collapse-docs \
	    --passes unindent-comments \
	    --passes strip-priv-imports

.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: codegen
codegen:
	cd $(HERE)/codegen/ && $(MAKE) && cd $(HERE)
