
#[derive(Debug, Clone)]
pub struct TextSpan {
  pub start: usize,
  pub len: usize,
  pub end: usize
}


impl TextSpan {
  pub fn new(start: usize, len: usize) -> Self {
    Self {
      start,
      len,
      end: start + len
    }
  }
}
