ErlangRT - Runtime
==================

Erlang Replacement Therapy.
Another attempt to make Erlang runtime (BEAM emulator) in Rust.

* The good news: I know what to do.
* The bad news: I have no clue how to Rust, but will learn.

Progress to the Proof of Concept
--------------------------------

* Term library 60%
* External Term Format (decoder 40%)
* BEAM Loader 95%
* VM and processes 25%
* VM loop and opcodes 15%
* Some basic BIFs 0%

The following code already works:

.. code:: erlang

    -module(test).
    -export([start/0]).
    start() ->
        test1([1, 2, 3, 4]).
    test1(X) -> lists:reverse(X).

And would print:

.. code::

    [exec] 0x7f44396f65a0: move X(1), X(0)
    [exec] 0x7f44396f65b8: return
    thread 'main' panicked at 'opcodes::op_execution: Process exit: normal; x0=[4, 3, 2, 1]', src/beam/opcodes/op_execution.rs:83:6


Compiling
---------

* Install latest **Rust** and **Cargo** via `Rustup <http://doc.crates.io/>`_
* Run ``make`` and with the magic of Bash autocomplete see which targets it
  supports. You might like:
    * ``make run`` - runs the executable with test args, whatever set by the developer,
      do not expect it to show any magical tricks;
    * ``make doc`` - builds doc pages in ``target/doc/erlang_rt/``
    * ``make test`` - runs the tests
    * ``make build`` and ``make build-rel`` - builds but does not run the debug and
      the release target respectively

Editing and Code Navigation
---------------------------

I am using and strongly recommend IntelliJ IDEA CE (free version) with
IntelliJ-Rust plugin (available in repositories tab inside IntelliJ).

Reference Material
------------------

* `BEAM Wisdoms <http://beam-wisdoms.clau.se/>`_ (I run this one)
* `The BEAM book <https://github.com/happi/theBeamBook>`_
  (I am also one of the editors there)
