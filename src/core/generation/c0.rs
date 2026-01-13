use crate::core::{generation::Generator, tokenizer::Token};

impl Generator {
  pub fn left(&mut self, amount: u8) {
    for _ in 0..amount {
      self.pointer = self.pointer.wrapping_sub(1);
      self.push(Token::LeftAngle);
    }
  }
  pub fn right(&mut self, amount: u8) {
    for _ in 0..amount {
      self.pointer = self.pointer.wrapping_add(1);
      self.push(Token::RightAngle);
    }    
  }
  pub fn add(&mut self, val: u8) {
    for _ in 0..val {
      self.push(Token::Plus);
    }
  }
  pub fn sub(&mut self, val: u8) {
    for _ in 0..val {
      self.push(Token::Minus);
    }
  }
  pub fn putchar(&mut self) {
    self.push(Token::Dot);
  }
  pub fn getchar(&mut self) {
    self.push(Token::Comma);
  }
  pub fn create_label(&mut self, name: &str) {
    self.push(Token::LabelDef);
    self.push(Token::Identifier(name.to_string()));
  }
  pub fn jump(&mut self, name: &str) {
    self.push(Token::Jmp);
    self.push(Token::Identifier(name.to_string()));
  }
  pub fn jze(&mut self, name: &str) {
    self.push(Token::Jze);
    self.push(Token::Identifier(name.to_string()));
  }
  pub fn jnze(&mut self, name: &str) {
    self.push(Token::Jnze);
    self.push(Token::Identifier(name.to_string()));
  }
  pub fn mul(&mut self) {
    self.push(Token::Star);
  }
  pub fn div(&mut self) {
    self.push(Token::Slash);
  }
  pub fn invert(&mut self) {
    self.push(Token::Exclamation);
  }
}
