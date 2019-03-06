.PHONY: build codegen ct

build: codegen
	cargo build

build_tests:
	cd priv && $(MAKE)

codegen:
	cd lib-erlangrt && $(MAKE) codegen

ct: build
	mkdir tmp; cd tmp && ../target/debug/ct_run 1 2 3 -erl_args 4 5 6

run: build build_tests
	cargo run --bin erlexec

test: build build_tests
	RUST_BACKTRACE=1 cargo run --bin ct_run

# Graphical user inteface for GDB - Gede
.PHONY: test-gede
test-gede: build
	RUST_BACKTRACE=1 gede --args target/debug/ct_run
