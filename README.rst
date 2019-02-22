ErlangRT - Runtime
==================

Erlang Replacement Therapy.
This is an attempt to make Erlang runtime (BEAM emulator) in Rust. This is not
the first attempt so I kinda know what I'm doing.

Progress to the Proof of Concept
--------------------------------

* Term library 75%
* External Term Format (decoder 70%, encoder 0%)
* BEAM Loader 95%
* VM and processes 35%
* VM loop and opcodes 25%
    * Exceptions: basic support, try/catch works
* Some basic BIFs 7%

.. figure:: https://i.imgur.com/1ryd4K1.png
   :scale: 50 %
   :alt: Early Test2.erl run output

.. figure:: http://imgur.com/H5qypZG.png
   :scale: 50%
   :alt: Trying to run ``init:boot/1``

Tests in ``priv/test2.erl`` work. Running ``make test`` tries to run ``init:boot/1`` and produces the output above.

Compiling
`````````

* The source assumes that you have Erlang OTP 21 source in a directory ``../otp``
  relative to the ErlangRT directory, and the stdlib and preload BEAM files are
  compiled and ready.
* Install latest **Rust** and **Cargo** via `Rustup <http://doc.crates.io/>`_
* Run ``make`` and with the magic of Bash autocomplete see which targets it
  supports. You might like:
    * ``make run`` - runs the executable with test args, whatever set by the developer,
      do not expect it to show any magical tricks;
    * ``make doc`` - builds doc pages in ``target/doc/erlang_rt/``
    * ``make test`` - runs the tests
    * ``make build`` and ``make build-rel`` - builds but does not run the debug and
      the release target respectively
      
Currently the emulator expects to have preloaded BEAM modules from OTP 21 located in `../otp` relative
to the project directory.

Editing and Code Navigation
```````````````````````````

I am using and strongly recommend IntelliJ IDEA CE (free version) with
IntelliJ-Rust plugin (available in repositories tab inside IntelliJ).

Reference Material
``````````````````

* `BEAM Wisdoms <http://beam-wisdoms.clau.se/>`_ (I run this one)
* `The BEAM book <https://github.com/happi/theBeamBook>`_
  (I am also one of the editors there)

Contributing
````````````

See ``CONTRIBUTING.rst``
