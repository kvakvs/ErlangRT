use crate::{
  beam::{
    gen_op,
    loader::{compact_term, fterm::FTerm, op_badarg_panic, LoaderState, PatchLocation},
  },
  defs::{Arity, Word},
  emulator::{
    code::{opcode, CodeOffset, LabelId},
    funarity::FunArity,
  },
  fail::RtResult,
  rt_util::bin_reader::BinaryReader,
  term::{boxed, lterm::LTerm},
};

fn module() -> &'static str {
  "beam/loader/parsecode: "
}

impl LoaderState {
  /// Assume that loader raw structures are completed, and atoms are already
  /// transferred to the VM, we can now parse opcodes and their args.
  /// 'drained_code' is 'raw_code' moved out of 'self'
  pub fn parse_raw_code(&mut self) -> RtResult<()> {
    // Dirty swap to take raw_code out of self and give it to the binary reader
    let mut raw_code: Vec<u8> = Vec::new();
    core::mem::swap(&mut self.raw.code, &mut raw_code);

    // Estimate code size and preallocate the code storage
    // TODO: This step is not efficient and does double parse of all args
    //
    let mut r = BinaryReader::from_bytes(raw_code);

    let code_size = {
      let mut s = 0usize;
      while !r.eof() {
        let op = opcode::RawOpcode(r.read_u8());
        let arity = gen_op::opcode_arity(op) as usize;
        for _i in 0..arity {
          let _arg0 = compact_term::read(&mut r).unwrap();
        }
        s += arity + 1;
      }
      s
    };
    self.code.reserve(code_size);

    let debug_code_start = self.code.as_ptr();
    // println!("Code_size {} code_start {:p}", code_size, debug_code_start);

    // Writing code unpacked to words here. Break at every new function_info.
    //
    r.reset();
    while !r.eof() {
      // Read the opcode from the code section
      let op = opcode::RawOpcode(r.read_u8());

      // Read `arity` args, and convert them to reasonable runtime values
      let arity = gen_op::opcode_arity(op);
      let mut args: Vec<FTerm> = Vec::new();
      for _i in 0..arity {
        let arg0 = compact_term::read(&mut r).unwrap();
        // Atom_ args now can be converted to Atom (VM atoms)
        let arg1 = match self.resolve_loadtime_values(&arg0) {
          Some(tmp) => tmp,
          None => arg0,
        };
        args.push(arg1);
      }

      match op {
        // add nothing for label, but record its location
        gen_op::OPCODE_LABEL => {
          if let FTerm::SmallInt(f) = args[0] {
            // Store weak ptr to function and code offset to this label
            let floc = self.code.len();
            self.labels.insert(LabelId(f as Word), CodeOffset(floc));
          } else {
            op_badarg_panic(op, &args, 0);
          }
        }

        // add nothing for line, but TODO: Record line contents
        gen_op::OPCODE_LINE => {}

        // else push the op and convert all args to LTerms, also remember
        // code offsets for label values
        _ => {
          if op == gen_op::OPCODE_FUNC_INFO {
            // arg[0] mod name, arg[1] fun name, arg[2] arity
            let funarity = FunArity {
              f: args[1].to_lterm(&mut self.lit_heap, &self.lit_tab),
              arity: args[2].loadtime_word() as Arity,
            };

            // Function code begins after the func_info opcode (1+3)
            let fun_begin = self.code.len() + 4;
            if self.name.is_some() {
              self.funs.insert(funarity, fun_begin);
            } else {
              panic!("{}mod_id must be set at this point", module())
            }
          }

          self.code.push(opcode::to_memory_word(op));
          self.store_opcode_args(&args)?;
        } // case _
      } // match op
    } // while !r.eof

    assert_eq!(
      debug_code_start,
      self.code.as_ptr(),
      "{}Must do no reallocations",
      module()
    );
    Ok(())
  }


  /// Given arity amount of `args` from another opcode, process them and store
  /// into the `self.code` array. `LoadtimeExtList` get special treatment as a
  /// container of terms. `LoadtimeLabel` get special treatment as we try to
  /// resolve them into an offset.
  fn store_opcode_args(&mut self, args: &[FTerm]) -> RtResult<()> {
    for a in args {
      match *a {
        // Ext list is special so we convert it and its contents to lterm
        FTerm::LoadtimeExtlist(ref jtab) => {
          // Push a header word with length
          let heap_jtab = boxed::Tuple::create_into(&mut self.lit_heap, jtab.len())?;
          self.code.push(LTerm::make_boxed(heap_jtab).raw());

          // Each value convert to LTerm and also push forming a tuple
          for (index, t) in jtab.iter().enumerate() {
            let new_t = if let FTerm::LoadtimeLabel(f) = *t {
              // Try to resolve labels and convert now, or postpone
              let ploc =
                PatchLocation::PatchJtabElement(LTerm::make_boxed(heap_jtab), index);
              self.maybe_convert_label(LabelId(f), ploc)
            } else {
              t.to_lterm(&mut self.lit_heap, &self.lit_tab).raw()
            };

            unsafe { (*heap_jtab).set_element_raw(index, new_t) }
          }
        }

        // Label value is special, we want to remember where it was
        // to convert it to an offset
        FTerm::LoadtimeLabel(f) => {
          let ploc = PatchLocation::PatchCodeOffset(self.code.len());
          let new_t = self.maybe_convert_label(LabelId(f), ploc);
          self.code.push(new_t)
        }

        // Load-time literals are already loaded on `self.lit_heap`
        FTerm::LoadtimeLit(lit_index) => self.code.push(self.lit_tab[lit_index].raw()),

        // Otherwise convert via a simple method
        _ => self
          .code
          .push(a.to_lterm(&mut self.lit_heap, &self.lit_tab).raw()),
      }
    } // for a in args
    Ok(())
  }

  /// Given label index `l` check if it is known, then return a new jump
  /// destination - a boxed code location pointer to be used by the caller.
  /// Otherwise the `patch_location` is stored to `self.replace_labels` to be
  /// processed later and a `SmallInt` is returned to be used temporarily.
  fn maybe_convert_label(&mut self, l: LabelId, patch_loc: PatchLocation) -> Word {
    // Resolve the label, if exists in labels table
    match self.labels.get(&l) {
      Some(offset0) => self.create_jump_destination(*offset0),
      None => {
        self.replace_labels.push(patch_loc);
        let LabelId(label_id) = l;
        LTerm::make_small_unsigned(label_id).raw()
      }
    }
  }
}
