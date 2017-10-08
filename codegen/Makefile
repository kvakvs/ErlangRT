.PHONY: all
all: genop genatoms
#vm_loop vm_predef_atoms vm_bif_tab

SRCDIR=../src

.PHONY: genop
genop:
	./create_gen_op.py > ${SRCDIR}/beam/gen_op.rs

.PHONY: genatoms
genatoms:
	./create_gen_atoms.py > ${SRCDIR}/emulator/gen_atoms.rs
