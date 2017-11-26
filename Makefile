HERE=$(shell pwd)

.PHONY: run run-rel
# Compile and run
run: priv
	cargo run
#	target/debug/erlang_rt

run-rel: priv
	cargo run --release

# Run with backtrace on
.PHONY: run-bt
run-bt: priv
	RUST_BACKTRACE=1 cargo run

# Build test modules from priv/
.PHONY: priv
priv:
	mkdir priv; cd priv && $(MAKE)

.PHONY: gdb
gdb: build
	gdb target/debug/erlang_rt

.PHONY: build build-rel asm asm-rel
build:
	cargo build

build-rel:
	cargo build --release

asm:
	cargo rustc -- --emit asm

asm-rel:
	cargo rustc --release -- --emit asm

.PHONY: clippy
clippy:
	cargo rustc --features clippy -- -Z no-trans -Z extra-plugins=clippy

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
