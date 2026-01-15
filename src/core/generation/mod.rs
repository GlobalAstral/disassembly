use std::collections::HashMap;

use crate::core::{error::DSAsmError, interpreter::Interpreter, parser::Node, processor::{Processor, ProcessorInput}, tokenizer::Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Cell {
  #[default]
  Unused,
  Used,
  Variable(u64),
  Temporary,
  Return,
  Parameter(u64)
}

impl Cell {
  pub fn is_unused(&self) -> bool {
    match self {
      Self::Unused => true,
      _ => false
    }
  }
  pub fn to_string(&self) -> String {
    match self {
      Cell::Temporary => "Temp".into(),
      Cell::Unused => "Unus".into(),
      Cell::Used => "Used".into(),
      Cell::Variable(id) => format!("#{:03}", id),
      Cell::Parameter(id) => format!("§{:03}", id),
      Cell::Return => "Rtrn".into(),
    }
  }
  pub fn is_used(&self) -> bool {
    match self {
      Self::Used => true,
      _ => false
    }
  }
  pub fn is_variable(&self) -> bool {
    match self {
      Self::Variable(_) => true,
      _ => false
    }
  }
  pub fn is_variable_of_id(&self, id: u64) -> bool {
    match self {
      Self::Variable(i) => *i == id,
      _ => false
    }
  }
  pub fn is_temp(&self) -> bool {
    match self {
      Self::Temporary => true,
      _ => false
    }
  }
  pub fn get_var_id(&self) -> Option<u64> {
    match self {
      Cell::Variable(id) => Some(*id),
      _ => None
    }
  } 
}

pub type Stack = [Cell; Interpreter::STACK_SIZE];

const EMPTY_STACK: Stack = [Cell::Unused; Interpreter::STACK_SIZE];

impl ProcessorInput for Node { }

pub struct Generator {
  base: Processor<Node>,
  stack: Stack,
  pointer: u8,
  free_cache: Vec<u8>,
  output: Vec<Token>,
}

impl Generator {
  pub fn new(i: Vec<Node>) -> Generator {
    Generator { base: Processor::new(i), stack: EMPTY_STACK, pointer: 0, free_cache: Vec::new(), output: Vec::new() }
  }

  pub fn print_memory(&self) {
    let side: usize = (Interpreter::STACK_SIZE as f32).sqrt() as usize;
    let hex_size_len: usize = std::mem::size_of::<u8>() * 2 + 2;
    self.stack.chunks(side).enumerate().for_each(|(addr, value)| {
      let temp: String = value.iter().map(|u: &Cell| format!("{:^4}", u.to_string())).collect::<Vec<String>>().join(" | ") + " |";
      println!("{:#0hex_size_len$X} | {}", addr * value.len(), temp);
    });
  }

  fn push(&mut self, t: Token) {
    self.output.push(t);
  }

  pub fn generate_all(&mut self) -> Result<Vec<Token>, DSAsmError> {
    while self.base.has_peek() {
      let node = self.base.consume();
      self.generate(&node)?;
    }

    Ok(self.output.clone())
  }
}

mod c0;
mod c1;
mod c2;
