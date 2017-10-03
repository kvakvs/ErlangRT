# takes: genop.tab from erlang/otp
# returns list of dicts{name:str(), arity:int(), opcode:int()}

import string
from typing import *


class OTPConfig:
    """ Defines rules for parsing different OTP version inputs """

    def __init__(self, min_opcode: int, max_opcode: int,
                 atoms_tab: str, bif_tab: str, genop_tab: str):
        self.min_opcode = min_opcode
        self.max_opcode = max_opcode
        self.atoms_tab = atoms_tab
        self.bif_tab = bif_tab
        self.genop_tab = genop_tab

    def parse_bif_line(self, b): ...


class OTP19(OTPConfig):
    def __init__(self):
        super().__init__(min_opcode=1, max_opcode=158,
                         atoms_tab="otp19/atoms.tab",
                         bif_tab="otp19/bif.tab",
                         genop_tab="otp19/genop.tab")

    def parse_bif_line(self, b):
        b = b.split()
        if len(b) >= 3:
            cname = b[2]
        else:
            cname = b[0]
        return Bif(atom=b[0],
                   arity=int(b[1]),
                   cname=cname)


class OTP20(OTPConfig):
    def __init__(self):
        super().__init__(min_opcode=1, max_opcode=159,
                         atoms_tab="otp20/atom.names",
                         bif_tab="otp20/bif.tab",
                         genop_tab="otp20/genop.tab")

    def parse_bif_line(self, line):
        line = line.split()
        btype = line[0]
        (mod, funarity) = line[1].split(':', 1)
        (fun, arity) = funarity.rsplit('/', 1)
        cname = line[2] if len(line) >= 3 else fun

        return Bif(atom=fun,
                   arity=arity,
                   cname=cname,
                   biftype=btype)


class Genop:
    def __init__(self, name: str, arity: int, opcode: int):
        self.name = name
        self.arity = arity
        self.opcode = opcode

    def cname(self):
        s_parts = self.name.split("_")
        return "".join([s.capitalize() for s in s_parts])


class Bif:
    def __init__(self, atom: str, arity: int, cname: int, biftype=None):
        self.atom = atom
        self.arity = arity
        self.cname = cname
        self.biftype = biftype  # None, ubif (no heap), gcbif (use heap), bif


class Atom:
    def __init__(self, atom: str, cname: Union[str, None]):
        self.cname = cname
        self.text = atom
        self.id = None


class OTPTables:
    """ Class handles loading tables from OTP source, used for code generation
        by scripts in `codegen/`
    """

    def __init__(self, conf: OTPConfig):
        self.conf = conf
        self.ops = {}  # type: Dict[int, Genop]
        self.implemented_ops = OTPTables.filter_comments(
            open("implemented_ops.tab").read().split("\n"))

        self.bif_tab = []

        self.atom_tab = []  # type: List[Atom]
        self.atom_id = 1
        # maps atom string to integer
        self.atom_id_tab = {}  # type: Dict[str, int]
        # Dict[int, {atom, id}] - maps atom id to atom record
        self.id_atom_tab = {}  # type: Dict[int, Atom]

        self.load_opcodes()
        self.load_bifs()

    def load_opcodes(self):
        """ Read the GENOP_TAB file and produce a dict of ops
        """
        for ln in open(self.conf.genop_tab).readlines():
            ln = ln.strip()
            if not ln:
                continue
            if ln.startswith("#"):
                continue

            p1 = ln.split(" ")
            if len(p1) != 2:
                continue

            opcode = int(p1[0].strip(":"))
            (op_name, op_arity) = p1[1].split("/")
            op_name = op_name.strip("-")
            self.ops[opcode] = Genop(name=op_name,
                                     arity=int(op_arity),
                                     opcode=opcode)

            # Don't remember where these 3 extra codes go, legacy of gluonvm1
            # max_opcode = conf.max_opcode
            # extra_codes = 3
            # ops[max_opcode + 1] = Genop(name='normal_exit_',
            #                             arity=0,
            #                             opcode=max_opcode + 1)
            # ops[max_opcode + 2] = Genop(name='apply_mfargs_',
            #                             arity=0,
            #                             opcode=max_opcode + 2)
            # ops[max_opcode + 3] = Genop(name='error_exit_',
            #                             arity=0,
            #                             opcode=max_opcode + 3)
            # max_opcode += extra_codes

    @staticmethod
    def filter_comments(lst):
        # skip lines starting with # and empty lines
        return [i for i in lst
                if not i.strip().startswith("#") and len(i.strip()) > 0]

    @staticmethod
    def is_printable(s):
        printable = string.ascii_letters + string.digits + "_"
        for c in s:
            if c not in printable:
                return False
        return True

    @staticmethod
    def bif_cname(b):
        if len(b) >= 3:
            return b[2]
        else:
            return b[0]

    @staticmethod
    def atom_constname(a):
        if 'cname' in a:
            return "Q_" + a['cname'].upper()
        else:
            return a['atom'].upper()

    def atom_add(self, a: Atom):
        if a.text in self.atom_id_tab:  # exists
            return
        a.id = self.atom_id
        self.atom_tab.append(a)

        self.atom_id_tab[a.text] = self.atom_id  # name to id map
        self.id_atom_tab[self.atom_id] = a  # id to atom map
        self.atom_id += 1

    def load_bifs(self):
        atoms = self.filter_comments(
            open(self.conf.atoms_tab).read().split("\n"))

        for a in atoms:
            self.atom_add(Atom(atom=a, cname=None))

        bifs = self.filter_comments(
            open(self.conf.bif_tab).read().split("\n"))
        bif_tab0 = []
        for bline in bifs:
            bif = self.conf.parse_bif_line(bline)
            bif_tab0.append(bif)

            if self.is_printable(bline[0]):
                self.atom_add(Atom(atom=bline[0], cname=None))
            else:
                self.atom_add(Atom(atom=bline[0], cname=bif.cname))

        # sort by (atom_text, arity) if atom ids equal
        self.bif_tab = sorted(
            bif_tab0,
            key=lambda b0: (b0.atom, b0.arity)
        )
