use crate::{
  beam::loader::LoaderState,
  emulator::{atom, code_srv::CodeServer, function::FunEntry, mfa::ModFunArity},
};

fn module() -> &'static str {
  "beam/loader/stage2: "
}

impl LoaderState {
  pub fn stage2_register_atoms(&mut self, code_server: &mut CodeServer) {
    self.vm_atoms.reserve(self.beam_file.atoms.len());
    for a in &self.beam_file.atoms {
      self.vm_atoms.push(atom::from_str(a));
    }

    // Create a new version number for this module and fill self.mod_id
    self.set_mod_id(code_server)
  }

  pub fn stage2_fill_lambdas(&mut self) {
    // Convert LFuns in self.raw.funs to FunEntries
    for rf in &self.beam_file.lambdas {
      let fun_name = self.atom_from_loadtime_index(rf.fun_atom_i);
      let mfa = ModFunArity::new(self.module_name(), fun_name, rf.arity);
      println!("{}stage2_fill_lambdas mfa={}", module(), mfa);
      self.lambdas.push(FunEntry::new(mfa, rf.nfrozen))
    }
  }
}
