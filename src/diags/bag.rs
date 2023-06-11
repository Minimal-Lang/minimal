use crate::{
  diags::diag::Diag,
  lexer::token::TokenKind,
  text::text_span::TextSpan
};


pub struct DiagBag {
  pub diags: Vec<Diag>
}

impl DiagBag {
  pub fn new() -> Self {
    Self {
      diags: Vec::<Diag>::new()
    }
  }

  fn report(&mut self, span: TextSpan, msg: String) {
    self.diags.push(
      Diag::new(span, msg)
    );
  }


  pub fn report_illegal_ch(&mut self, pos: usize, ch: char) {
    let span = TextSpan::new(pos, 1);
    let msg = format!("Illegal character input: '{}'", ch);
    self.report(span, msg);
  }

  pub fn report_unexpected_token(&mut self, span: TextSpan, kind: TokenKind, expected: TokenKind) {
    let msg = format!("Unexpected token <{:?}>, expected <{:?}>", kind, expected);
    self.report(span, msg);
  }
}
