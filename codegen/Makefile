.PHONY: all
all: genop
#vm_loop vm_predef_atoms vm_bif_tab

SRCDIR=../src

.PHONY: genop
genop:
	./create_gen_op.py > ${SRCDIR}/beam/gen_op.rs

#.PHONY: vm_loop
#vm_loop:
#	./create_vm_loop.py > ${SRCDIR}/vm_loop.inc.cpp

#.PHONY: vm_predef_atoms
#vm_predef_atoms:
#	./create_predef_atoms_h.py > ${INCDIR}/predef_atoms.h && \
#	./create_predef_atoms_cpp.py > ${SRCDIR}/predef_atoms.cpp

#.PHONY: vm_bif_tab
#vm_bif_tab:
#	./create_bif_tab_h.py > ${SRCDIR}/bif_tab.h && \
#	./create_bif_tab_cpp.py > ${SRCDIR}/bif_tab.cpp
