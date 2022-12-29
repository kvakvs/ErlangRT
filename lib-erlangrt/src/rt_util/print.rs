/// Dump vector contents as hex table
///
/// `0000 | 61 21 30 00 00 00 00 00 | 00 00 00 00 00 00 00 00  a!0..... ........`
#[allow(dead_code)]
pub fn dump_vec(data: &[u8]) {
  let mut i = 0;
  while i < data.len() {
    // Offset
    print!("{i:04x} | ");

    // Print hex bytes
    for j in 0..16 {
      if j == 8 {
        print!("| ")
      }
      if i + j >= data.len() {
        print!("   ");
        continue;
      }
      print!("{:02x} ", data[i + j])
    }
    print!("  ");

    // Print ASCII repr
    for j in 0..16 {
      if i + j >= data.len() {
        break;
      }
      if j == 8 {
        print!(" ")
      }
      let c = data[i + j];
      if (32..127).contains(&c) {
        print!("{}", c as char)
      } else {
        print!(".")
      }
    }
    println!();
    i += 16
  }
}
