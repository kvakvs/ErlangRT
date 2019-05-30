use crate::{
  beam::{gen_op, loader::LoaderState},
  emulator::{
    code::{self, opcode, CodePtrMut},
    function::FunEntry,
    mfa::ModFunArity,
  },
  fail::RtResult,
  term::{boxed, Term},
};

impl LoaderState {
  /// Analyze the code and for certain opcodes overwrite their import index
  /// args with direct pointer to import heap.
  pub fn setup_imports(&mut self) -> RtResult<()> {
    // Step 1
    // Write imports onto literal heap as {Mod, Fun, Arity} triplets
    //
    self.imports.reserve(self.beam_file.imports.len());
    for ri in &self.beam_file.imports {
      let mod_atom = self.atom_from_loadtime_index(ri.mod_atom_i);
      let fun_atom = self.atom_from_loadtime_index(ri.fun_atom_i);
      let mf_arity = ModFunArity::new(mod_atom, fun_atom, ri.arity);
      // println!("is_bif {} for {}", is_bif, mf_arity);
      let boxed_import =
        unsafe { boxed::Import::create_into(&mut self.beam_file.lit_heap, mf_arity)? };

      self.imports.push(boxed_import);
    }

    // Step 2
    // For each opcode if it has import index arg - overwrite it
    //
    let c_iter = unsafe { code::iter::create_mut(&mut self.code) };
    for cp in c_iter {
      let curr_opcode = opcode::from_memory_ptr(cp.ptr());
      match curr_opcode {
        gen_op::OPCODE_MAKE_FUN2 => self.rewrite_lambda_index_arg(cp, 1),
        gen_op::OPCODE_BIF0 => self.rewrite_import_index_arg(cp, 1),
        gen_op::OPCODE_BIF1
        | gen_op::OPCODE_BIF2
        | gen_op::OPCODE_CALL_EXT
        | gen_op::OPCODE_CALL_EXT_LAST
        | gen_op::OPCODE_CALL_EXT_ONLY => {
          // arg[1] is export
          self.rewrite_import_index_arg(cp, 2)
        }
        gen_op::OPCODE_GC_BIF1 | gen_op::OPCODE_GC_BIF2 | gen_op::OPCODE_GC_BIF3 => {
          // arg[2] is export
          self.rewrite_import_index_arg(cp, 3)
        }
        _ => {}
      }
    }
    Ok(())
  }

  /// Internal helper which takes N'th arg of an opcode, parses it as a small
  /// unsigned and writes an Term pointer to a literal {M,F,Arity} tuple.
  fn rewrite_import_index_arg(&self, cp: CodePtrMut, n: usize) {
    let import0 = unsafe { Term::from_raw(cp.read_n(n)) };
    let import1 = self.imports[import0.get_small_unsigned()];
    unsafe { cp.write_n(n, import1.raw()) }
  }

  /// Given a pointer to a `make_fun2` or similar opcode with a lambda index
  /// argument, replace it with a raw pointer to a loaded `FunEntry`.
  /// The `FunEntry` will be owned by the module we're loading, and will be
  /// freed together with the code, so it should be safe to use the pointer.
  fn rewrite_lambda_index_arg(&self, cp: CodePtrMut, n: usize) {
    let lambda_i = unsafe { Term::from_raw(cp.read_n(n)) };
    let lambda_p = &self.lambdas[lambda_i.get_small_unsigned()] as *const FunEntry;
    unsafe { cp.write_n(n, Term::make_cp(lambda_p).raw()) }
  }
}
