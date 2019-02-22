.PHONY: all
all: genop genatoms vmdisp genbif

SRCDIR=../src

.PHONY: genop
genop:
	./create_gen_op.py > ${SRCDIR}/beam/gen_op.rs

.PHONY: genatoms
genatoms:
	./create_gen_atoms.py > ${SRCDIR}/emulator/gen_atoms.rs

.PHONY: genbif
genbif:
	./create_gen_native_fun.py > ${SRCDIR}/native_fun/gen_native_fun.rs

.PHONY: vmdisp
vmdisp:
	./create_vm_dispatch.py > ${SRCDIR}/beam/vm_dispatch.rs
