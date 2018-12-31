While the project is young and far from Proof of Concept stage,
please keep your contributions to a minimum: something which will
not be changing a lot of code or the architecture.

Welcome are: style fixes, bug fixes and fix ideas, clippy warning 
fixes, typos and formatting etc.

I will be updating these guidelines once the project reaches some
sort of stability.

Implementing new BIFs
`````````````````````

1. Visit ``codegen/otp20/bif.tab`` and find the BIF you want to add, copy the line.
2. Visit ``codegen/implemented_bifs.tab`` file, and insert the line.
3. Run ``make`` in root directory once, which will invoke the codegen and show you errors about a missing function.
4. Copy from another bif, and satisfy the compiler.

Implementing new Opcodes
````````````````````````

1. Visit ``codegen/otp20/genop.tab`` and find the opcode you want to implement.
2. Visit ``codegen/implemented_ops.tab`` file, and insert what you intend to add.
3. Run ``make`` in root directory once, which will invoke the codegen and show you errors about a missing struct.
4. Copy from another opcode, edit the arity, edit the arguments and make it work.
