use term::boxed::BoxHeader;

pub enum BoxBinaryPayload {
  // contains size, followed in memory by the data bytes
  ProcBin { nbytes: Word } ,
  // contains reference to heapbin
  RefBin,
  // stores data on a separate heap somewhere else with refcount
  HeapBin { nbytes: Word, refc: Word },
}

/// Binary which stores everything in its allocated memory on process heap.
#[allow(dead_code)]
pub struct Binary {
  header: BoxHeader,
  pub payload: BoxBinaryPayload,
}

impl Binary {
}

