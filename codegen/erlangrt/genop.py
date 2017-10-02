# takes: genop.tab from erlang/otp
# returns list of dicts{name:str(), arity:int(), opcode:int()}

import string

MIN_OPCODE = 1
MAX_OPCODE = 158
ATOMS_TAB = "otp19/atoms.tab"
BIF_TAB = "otp19/bif.tab"
GENOP_TAB = "otp19/genop.tab"


def load_opcodes():
    global ops
    ops = []
    for ln in open(GENOP_TAB).readlines():
        ln = ln.strip()
        if not ln:
            continue
        if ln.startswith("#"):
            continue

        p1 = ln.split(" ")
        if len(p1) != 2:
            continue

        opcode = p1[0].strip(":")
        (opname, oparity) = p1[1].split("/")
        opname = opname.strip("-")
        ops.append(
            {'name': opname, 'arity': int(oparity), 'opcode': int(opcode)})

    global MAX_OPCODE
    extra_codes = 3
    ops.append({'name': 'normal_exit_', 'arity': 0, 'opcode': MAX_OPCODE + 1})
    ops.append({'name': 'apply_mfargs_', 'arity': 0, 'opcode': MAX_OPCODE + 2})
    ops.append({'name': 'error_exit_', 'arity': 0, 'opcode': MAX_OPCODE + 3})
    MAX_OPCODE += extra_codes

    # make op map by opcode
    global ops_by_code
    ops_by_code = {}
    for op in ops:
        ops_by_code[op['opcode']] = op


def filter_comments(lst):
    # skip lines starting with # and empty lines
    return [i for i in lst
            if not i.strip().startswith("#") and len(i.strip()) > 0]


implemented_ops = filter_comments(
    open("implemented_ops.tab").read().split("\n"))
atom_tab = []
bif_tab = []
atom_id_tab = {}  # string() -> int()         - maps atom string to integer
id_atom_tab = {}  # int() -> dict({atom, id}) - maps atom id to atom record


def is_printable(s):
    printable = string.ascii_letters + string.digits + "_"
    for c in s:
        if c not in printable:
            return False
    return True


def bif_cname(b):
    if len(b) >= 3:
        return b[2]
    else:
        return b[0]


def atom_constname(a):
    if 'cname' in a:
        return "Q_" + a['cname'].upper()
    else:
        return a['atom'].upper()


atom_id = 1


def atom_add(a):
    global atom_tab, atom_id, atom_id_tab, id_atom_tab
    if a['atom'] in atom_id_tab:  # exists
        return
    adict = a
    adict['id'] = atom_id
    atom_tab.append(adict)
    atom_id_tab[a['atom']] = atom_id  # name to id map
    id_atom_tab[atom_id] = a  # id to atom map
    atom_id += 1


def load_bifs():
    global bif_tab, atom_tab
    atoms = filter_comments(open(ATOMS_TAB).read().split("\n"))
    for a in atoms:
        atom_add({'atom': a})

    bifs = filter_comments(open(BIF_TAB).read().split("\n"))
    bif_tab0 = []
    for b in bifs:
        b = b.split()
        if len(b) >= 3:
            cname = b[2]
        else:
            cname = b[0]
        bif_tab0.append({'atom': b[0], 'arity': int(b[1]), 'cname': cname})

        if is_printable(b[0]):
            atom_add({'atom': b[0]})
        else:
            atom_add({'atom': b[0], 'cname': cname})

    global atom_id_tab
    # sort by atom id plus arity if atom ids equal
    bif_tab = sorted(bif_tab0,
                     key=lambda b0: atom_id_tab[b0['atom']] * 1000 + b0[
                         'arity'])


def load():
    load_opcodes()
    load_bifs()


ops = []
ops_by_code = {}
