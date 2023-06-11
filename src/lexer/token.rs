use crate::text::text_span::TextSpan;


#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Illegal,
  EOF,

  // Identifiers and literals
  Identifier,
  Number,

  // Operators
  Equals,
  Plus,
  Minus,
  Star,
  Slash,

  // Delimiters
  OpenParen,
  CloseParen
}


#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub literal: String,
  pub pos: usize,
  pub len: usize,
  pub span: TextSpan
}

impl Token {
  pub fn new(kind: TokenKind, literal: String, pos: usize, len: usize) -> Self {
    Self {
      kind,
      literal,
      pos,
      len,
      span: TextSpan::new(pos, len)
    }
  }

  pub fn get_unary_op_precedence(kind: TokenKind) -> i32 {
    match kind {
      TokenKind::Plus |
      TokenKind::Minus => 3,
      _ => 0
    }
  }

  pub fn get_binary_op_precedence(kind: TokenKind) -> i32 {
    match kind {
      TokenKind::Star |
      TokenKind::Slash => 2,

      TokenKind::Plus |
      TokenKind::Minus => 1,

      _ => 0
    }
  }
}
