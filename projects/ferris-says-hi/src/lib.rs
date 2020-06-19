use ferris_says::say;
use std::io::{stdout, BufWriter};

#[no_mangle]
pub unsafe extern "C" fn fs_ferris_says_hi() {
  ferris_says_hi();
}

pub fn ferris_says_hi() {
  let hi = "Hi，我是 Ferris，是 Rustacean 的好朋友，很高兴认识你~";
  let stdout = stdout();
  let mut writer = BufWriter::new(stdout.lock());
  let _ = say(hi.as_bytes(), 50, &mut writer);
}
