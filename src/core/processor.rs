use std::{error::Error, fmt::{Debug, Display}};

use crate::core::error::DSAsmError;

pub trait ProcessorInput: Display + Debug + Default + Clone + Eq + PartialEq { }

pub struct Processor<I: ProcessorInput> {
  input: Vec<I>,
  peek: usize
}

impl<I: ProcessorInput> Processor<I>  {
  pub fn new(i: Vec<I>) -> Processor<I> {
    Processor { input: i, peek: 0 }
  }
  pub fn has_peek(&self) -> bool {
    self.peek < self.input.len()
  }
  pub fn peek(&self) -> I {
    if self.has_peek() {
      self.input.get(self.peek).unwrap().clone()
    } else {
      I::default()
    }
  }
  pub fn consume(&mut self) -> I {
    if self.has_peek() {
      let ret = self.peek();
      self.peek += 1;
      ret
    } else {
      I::default()
    }
  }
  pub fn peek_equal(&self, cmp: I) -> bool {
    self.peek() == cmp
  }
  pub fn tryconsume(&mut self, cmp: I) -> bool {
    if self.peek_equal(cmp) {
      self.consume();
      true
    } else {
      false
    }
  }
  pub fn require(&mut self, cmp: I) -> Result<I, Box<dyn Error>> {
    if self.peek_equal(cmp.clone()) {
      Ok(self.consume())
    } else {
      Err(DSAsmError::ProcessorError(format!("Expected '{}', peek={} - ", cmp, self.peek)).into())
    }
  }
  pub fn get_peek(&self) -> usize {
    self.peek
  }
  pub fn set_peek(&mut self, u: usize) {
    self.peek = u;
  }
  
}
