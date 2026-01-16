use crate::core::{generation::Generator, interpreter::{Interpreter, MemoryUnit}, tokenizer::Token};

impl Generator {
  pub fn goto(&mut self, addr: MemoryUnit) {
    self.pointer = addr;
    self.push(Token::Caret);
    self.push(Token::Literal(addr));
  }
  pub fn add(&mut self, val: MemoryUnit) {
    for _ in 0..val {
      self.push(Token::Plus);
    }
  }
  pub fn sub(&mut self, val: MemoryUnit) {
    for _ in 0..val {
      self.push(Token::Minus);
    }
  }
  pub fn goto_ins(&mut self, val: MemoryUnit) {
    self.push(Token::Minus);
    self.push(Token::RightAngle);
    self.push(Token::Literal(val));
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
  pub fn mul(&mut self, r: MemoryUnit) {
    self.push(Token::Star);
    self.push(Token::Literal(r));
  }
  pub fn div(&mut self, r: MemoryUnit) {
    self.push(Token::Slash);
    self.push(Token::Literal(r));
  }
  pub fn invert(&mut self) {
    self.push(Token::Exclamation);
  }
}
