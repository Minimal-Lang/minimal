use crate::text::text_span::TextSpan;


#[derive(Clone)]
pub struct Diag {
  pub span: TextSpan,
  pub msg: String
}


impl Diag {
  pub fn new(span: TextSpan, msg: String) -> Self {
    Self { span, msg }
  }
}
