use std::fmt::Display;

use crate::core::{error::DSAsmError, processor::{Processor, ProcessorInput}};

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub enum Token {
  LeftAngle,
  RightAngle,
  Plus,
  Minus,
  Dot,
  Comma,
  LabelDef,
  Jmp,
  Jze,
  Star,
  Slash,
  Exclamation,
  Identifier(String),

  Literal(u8),
  Ampersand,
  Tilde,
  Percent,
  Equals,
  Caret,
  Pipe,
  OpenParen,
  CloseParen,
  Let,
  OpenCurly,
  CloseCurly,
  If,
  While,
  For,
  Putchar,
  Return,
  Method,
  Getchar,
  Semicolon,

  #[default]
  Invalid
}

impl ProcessorInput for Token { }

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl ProcessorInput for char { }

pub struct Tokenizer {
  base: Processor<char>,
  line: usize,
}

impl Tokenizer {
  pub fn new(content: Vec<char>) -> Tokenizer {
    Tokenizer { 
      base: Processor::new(content), 
      line: 0
    }
  }

  pub fn tokenize(&mut self) -> Result<Vec<Token>, DSAsmError> {
    let mut ret: Vec<Token> = Vec::new();
    while self.base.has_peek() {
      let token: Token = match self.base.consume() {
        '>' => Token::RightAngle,
        '<' => Token::LeftAngle,
        '+' => Token::Plus,
        '-' => Token::Minus,
        '.' => Token::Dot,
        ',' => Token::Comma,
        ':' => Token::LabelDef,
        '@' => Token::Jmp,
        '?' => Token::Jze,
        '*' => Token::Star,
        '/' => Token::Slash,
        '!' => Token::Exclamation,
        '~' => Token::Tilde,
        '%' => Token::Percent,
        '=' => Token::Equals,
        '^' => Token::Caret,
        '|' => Token::Pipe,
        '&' => Token::Ampersand,
        '(' => Token::OpenParen,
        ')' => Token::CloseParen,
        '{' => Token::OpenCurly,
        '}' => Token::CloseCurly,
        ';' => Token::Semicolon,
        ch => {
          if ch == '\n' {
            self.line += 1;
            continue;
          }
          if ch.is_whitespace() {
            continue;
          }
          if ch.is_alphabetic() {
            let mut buf = String::from(ch);
            while self.base.peek().is_alphabetic() {
              buf.push(self.base.consume());
            };

            match buf.as_str() {
              "let" => Token::Let,
              "if" => Token::If,
              "while" => Token::While,
              "for" => Token::For,
              "putchar" => Token::Putchar,
              "return" => Token::Return,
              "method" => Token::Method,
              "getchar" => Token::Getchar,
              buf => {
                Token::Identifier(buf.to_string())
              }
            }
          } else if ch.is_digit(10) {
            let mut buf = String::from(ch);
            while self.base.peek().is_digit(10) {
              buf.push(self.base.consume());
            };
            let ret = buf.parse::<u8>().map_err(|e| Err::<u8, DSAsmError>(DSAsmError::ProcessorError(format!("{}", e)).into()))?;
            Token::Literal(ret)
          } else if ch == '0' && self.base.tryconsume('x') && self.base.peek().is_digit(16) {
            let mut buf = String::from(ch);
            while self.base.peek().is_digit(16) {
              buf.push(self.base.consume());
            };
            let ret: u8 = u8::from_str_radix(&buf, 16).map_err(|e| Err::<u8, DSAsmError>(DSAsmError::ProcessorError(format!("{}", e)).into()))?;
            Token::Literal(ret)
          } else {
            return Err(DSAsmError::ProcessorError(format!("Invalid token {}[{}]", ch, self.line)).into())
          }
        }
      };
      ret.push(token);
    }

    Ok(ret)
  }
}
