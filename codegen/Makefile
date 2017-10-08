.PHONY: all
all: genop genatoms vmdisp

SRCDIR=../src

.PHONY: genop
genop:
	./create_gen_op.py > ${SRCDIR}/beam/gen_op.rs

.PHONY: genatoms
genatoms:
	./create_gen_atoms.py > ${SRCDIR}/emulator/gen_atoms.rs

.PHONY: vmdisp
vmdisp:
	./create_vm_dispatch.py > ${SRCDIR}/beam/vm_dispatch.rs
