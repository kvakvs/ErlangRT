# takes: genop.tab from erlang/otp
# returns list of dicts{name:str(), arity:int(), opcode:int()}

import string
from sys import stderr
from typing import Union, List, Dict


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
                         atoms_tab="atoms.tab",
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
                         atoms_tab="atoms.tab",
                         bif_tab="implemented_bifs.tab",
                         genop_tab="otp20/genop.tab")

    def parse_bif_line(self, line):
        line = line.split()
        btype = line[0]
        (mod, funarity) = line[1].split(':', 1)
        (fun, arity) = funarity.rsplit('/', 1)
        cname = line[2] if len(line) >= 3 else fun

        return Bif(atom=fun,
                   mod=mod,
                   arity=arity,
                   cname=cname,
                   biftype=btype)


class OTP21(OTPConfig):
    def __init__(self):
        super().__init__(min_opcode=1, max_opcode=163,
                         atoms_tab="atoms.tab",
                         bif_tab="implemented_bifs.tab",
                         genop_tab="otp21/genop.tab")

    def parse_bif_line(self, line):
        line = line.split()
        btype = line[0]
        (mod, funarity) = line[1].split(':', 1)
        (fun, arity) = funarity.rsplit('/', 1)
        cname = line[2] if len(line) >= 3 else fun

        return Bif(atom=fun,
                   mod=mod,
                   arity=arity,
                   cname=cname,
                   biftype=btype)


class Genop:
    def __init__(self, name: str, arity: int, opcode: int):
        self.name = name
        self.arity = arity
        self.opcode = opcode


def enum_name(name: str) -> str:
    """ Capitalize all parts of a name to form a suitable enum name """
    if name.startswith("'"):
        return enum_name(name.strip("'"))

    s_parts = name.split("_")
    result = "".join([s.upper() for s in s_parts])
    return result


def c_fun_name(name: str) -> str:
    """ Capitalize all parts of a name to form a suitable enum name """
    if name.startswith("'"):
        return c_fun_name(name.strip("'"))

    return name.lower()


class Bif:
    def __init__(self, atom: str, arity: int, cname: str, mod: str,
                 biftype=None):
        self.arity = arity

        atom = atom.strip("'")
        self.atom_str = atom

        self.biftype = biftype  # None, ubif (no heap), gcbif (use heap), bif
        self.cname = cname.upper()
        self.mod = mod


class Atom:
    def __init__(self, atom: str, cname: Union[str, None]):
        self.cname = cname
        self.id = None
        self.text = atom


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
        self.atom_dict = {}  # type: Dict[str, Atom]
        self.atom_id = 1
        # maps atom string to integer
        self.atom_id_tab = {}  # type: Dict[str, int]
        self.id_atom_tab = {}  # type: Dict[int, Atom]

        self.load_opcodes()
        self.load_atoms_and_bifs()

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
        self.atom_dict[a.text] = a

        self.atom_id_tab[a.text] = self.atom_id  # name to id map
        self.id_atom_tab[self.atom_id] = a  # id to atom map
        self.atom_id += 1

    def load_atoms_and_bifs(self):
        """ Load the otp19 or otp20 bif.tab, depending on the self.conf
        """
        # Load atoms table and get rid of lines commented with '#'
        atoms = self.filter_comments(
            open(self.conf.atoms_tab).read().split("\n")
        )

        for a in atoms:
            a = a.split()
            if len(a) == 1:
                self.atom_add(Atom(atom=a[0], cname=a[0].upper()))
            else:
                new_a = Atom(atom=a[0], cname=a[1].upper())
                # stderr.write("atom/2 %s %s" % (new_a.text, new_a.cname))
                self.atom_add(new_a)

        # Load bifs table and get rid of lines commented with '#'
        bifs = self.filter_comments(
            open(self.conf.bif_tab).read().split("\n"))

        bif_tab0 = []  # type: List[Bif]
        for bline in bifs:
            # Parse bif.tab line with the help of the OTP version config
            # and accumulate Bif() objects in bif_tab0
            bif = self.conf.parse_bif_line(bline)
            bif_tab0.append(bif)

            # Add atom of function name
            self.atom_add(Atom(atom=bif.atom_str, cname=bif.cname))
            # Add atom of module name
            self.atom_add(Atom(atom=bif.mod, cname=bif.mod.upper()))

        # sort by (atom_text, arity) if atom ids equal
        self.bif_tab = sorted(
            bif_tab0,
            key=lambda b0: (b0.atom_str, b0.arity)
        )
