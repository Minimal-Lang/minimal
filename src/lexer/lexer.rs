use crate::{
  lexer::token::{ Token, TokenKind },
  diags::{
    diag::Diag,
    bag::DiagBag
  }
};


pub struct Lexer {
  source: Vec<char>,
  current: char,
  pos: usize,
  next: usize,
  diags: DiagBag
}

impl Lexer {
  pub fn new(contents: String) -> Self {
    Self {
      source: contents.chars().collect(),
      current: ' ',
      pos: 0,
      next: 0,
      diags: DiagBag::new()
    }
  }

  pub fn diags(&self) -> Vec<Diag> { return self.diags.diags.clone() }


  fn peek(&self) -> char {
    if self.next >= self.source.len() {
      return '\0';
    }

    self.source[self.next]
  }

  fn read(&mut self) -> char {
    if self.next >= self.source.len() {
      self.current = '\0';
    }
    else {
      self.current = self.source[self.next];
    }

    self.pos = self.next;
    self.next += 1;

    self.current
  }

  fn skip_whitespace(&mut self) {
    while self.peek().is_whitespace() {
      self.read();
    }
  }

  fn lex(&mut self) -> Token {
    self.skip_whitespace();
    self.read();

    let mut buf = String::from(self.current);
    let pos = self.pos;

    let test_id = |ch: char| ch.is_alphabetic() || ch == '_';
    let test_num = |ch: char| ch.is_numeric() || ch == '.';

    let kind = match self.current {
      '\0' => TokenKind::EOF,

      '=' => TokenKind::Equals,
      '+' => TokenKind::Plus,
      '-' => TokenKind::Minus,
      '*' => TokenKind::Star,
      '/' => TokenKind::Slash,

      '(' => TokenKind::OpenParen,
      ')' => TokenKind::CloseParen,

      _ if test_num(self.current) => {
        while test_num(self.peek()) {
          buf.push(self.read());
        }

        TokenKind::Number
      },

      _ if test_id(self.current) => {
        while test_id(self.peek()) {
          buf.push(self.read());
        }

        TokenKind::Identifier
      },

      _ => {
        self.diags.report_illegal_ch(pos, self.current);
        TokenKind::Illegal
      }
    };

    Token::new(kind, buf.clone(), pos, buf.len())
  }
}

impl Iterator for Lexer {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    if self.pos >= self.source.len() {
      return None;
    }

    Some(self.lex())
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tokens() {
    let suite = vec![
      (
        "123 * -(12 / -2)+6",
        vec![
          Token::new(TokenKind::Number, "123".to_owned(), 0, 3),
          Token::new(TokenKind::Star, "*".to_owned(), 4, 1),
          Token::new(TokenKind::Minus, "-".to_owned(), 6, 1),
          Token::new(TokenKind::OpenParen, "(".to_owned(), 7, 1),
          Token::new(TokenKind::Number, "12".to_owned(), 8, 2),
          Token::new(TokenKind::Slash, "/".to_owned(), 11, 1),
          Token::new(TokenKind::Minus, "-".to_owned(), 13, 1),
          Token::new(TokenKind::Number, "2".to_owned(), 14, 1),
          Token::new(TokenKind::CloseParen, ")".to_owned(), 15, 1),
          Token::new(TokenKind::Plus, "+".to_owned(), 16, 1),
          Token::new(TokenKind::Number, "6".to_owned(), 17, 1),
          Token::new(TokenKind::EOF, "\0".to_owned(), 18, 1)
        ],
        0
      )
    ];

    for case in suite {
      let (tokens, diags) = lexall(case.0);

      assert_eq!(diags.len(), case.2);
      assert_eq!(tokens.len(), case.1.len());

      for i in 0..tokens.len() {
        let res = tokens[i].clone();
        let expected = case.1[i].clone();

        assert_eq!(res.kind, expected.kind);
        assert_eq!(res.literal, expected.literal);
        assert_eq!(res.pos, expected.pos);
        assert_eq!(res.len, expected.len);
      }
    }
  }

  fn lexall(text: &str) -> (Vec<Token>, Vec<Diag>) {
    let mut lex = Lexer::new(text.to_owned());
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = lex.next() {
      tokens.push(token);
    }

    (tokens, lex.diags())
  }
}
