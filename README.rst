ErlangRT - Runtime
==================

Erlang Replacement Therapy.
Another attempt to make Erlang runtime (BEAM emulator) in Rust.

* The good news: I know what to do.
* The bad news: I have no clue how to Rust, but will learn.

Progress to the Proof of Concept
--------------------------------

* Term library 70%
* External Term Format (decoder 70%, encoder 0%)
* BEAM Loader 95%
* VM and processes 30%
* VM loop and opcodes 20%
* Some basic BIFs 5%

.. figure:: https://i.imgur.com/1ryd4K1.png
   :scale: 70 %
   :alt: Test2.erl run output

   Tests in ``priv/test2.erl`` partially work and would produce the output above.


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

Contributing
------------

Implementing new BIFs and Opcodes
`````````````````````````````````

1. Visit ``codegen/implemented_bifs.tab`` file or ``codegen/implemented_ops.tab``
   file, and insert what you intend to add. For opcodes it must also be present
   in ``codegen/otpXX/genop.tab`` because the VM will verify arities at least.
2. Run ``make`` in root directory once, which will invoke the codegen and show
   you errors about missing struct (for opcodes) or missing function (for bifs).
3. Copy from another opcode or a bif, and satisfy the compiler.
4. Profit.