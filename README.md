# ErlangRT - Runtime

Erlang Replacement Therapy.
Another attempt to make Erlang runtime (BEAM emulator) in Rust.

* The good news: I know what to do.
* The bad news: I have no clue how to Rust, but will learn.

## Project Progress

* Term library 30%
* BEAM Loader 90%
* VM and processes 20%

## Compiling

* Install latest **Rust** and **Cargo** via [Rustup](http://doc.crates.io/)
* Run `make` and with the magic of Bash autocomplete see which targets it
  supports. You might like:
    * `make run` - runs the executable with test args, whatever set by the developer,
      do not expect it to show any magical tricks;
    * `make doc` - builds doc pages in `target/doc/erlang_rt/`
    * `make test` - runs the tests
    * `make build` and `make build-rel` - builds but does not run the debug and
      the release target respectively

## Editing and Code Navigation

I am using and strongly recommend IntelliJ IDEA CE (free version) with
IntelliJ-Rust plugin (available in repositories tab inside IntelliJ).

## Reference Material

* [BEAM Wisdoms](http://beam-wisdoms.clau.se/)
* [The BEAM book](https://github.com/happi/theBeamBook)
  (I am also one of the editors there)
