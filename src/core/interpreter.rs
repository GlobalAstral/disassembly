use std::{collections::HashMap, error::Error, io::{Read, Write, stdin, stdout}};

use crate::core::{error::DSAsmError, processor::Processor, tokenizer::Token};


pub struct Interpreter {
  base: Processor<Token>,
  stack: [u8; Interpreter::STACK_SIZE],
  stack_ptr: usize,
  labels: HashMap<String, usize>
}

impl Interpreter {
  pub const STACK_SIZE: usize = 1024;
  pub fn new(content: Vec<Token>) -> Interpreter {
    Interpreter { 
      base: Processor::new(content),
      stack: [0; Interpreter::STACK_SIZE],
      stack_ptr: 0,
      labels: HashMap::new()
    }
  }

  fn get_identifier(&mut self) -> Result<String, DSAsmError> {
    match self.base.consume() {
      Token::Identifier(id) => Ok(id),
      t => {
        Err(DSAsmError::InterpreterError(format!("Expected Identifier instead of '{}'", t)).into())
      }
    }
  }

  fn label_must_not_exist(&self, name: &str) -> Result<(), DSAsmError> {
    if !self.labels.contains_key(&name.to_string()) {
      return Err(DSAsmError::InterpreterError(format!("Label '{}' does not exists", &name)).into());
    }
    Ok(())
  }

  pub fn interpret(&mut self) -> Result<(), DSAsmError> {
    while self.base.has_peek() {
      match self.base.consume() {
        Token::RightAngle => {
          self.stack_ptr = (self.stack_ptr + 1) % Interpreter::STACK_SIZE;
        },
        Token::LeftAngle => {
          self.stack_ptr = (self.stack_ptr - 1) % Interpreter::STACK_SIZE;
        },
        Token::Plus => {
          let tmp: u8 = self.stack[self.stack_ptr];
          self.stack[self.stack_ptr] = tmp.wrapping_add(1);
        },
        Token::Minus => {
          let tmp: u8 = self.stack[self.stack_ptr];
          self.stack[self.stack_ptr] = tmp.wrapping_sub(1);
        },
        Token::In => {
          stdout().flush().ok();
          let mut buf: [u8; 1] = [0];
          stdin().read_exact(&mut buf).expect("Cannot read user input");
          self.stack[self.stack_ptr] = buf[0];
        },
        Token::Out => {
          print!("{}", self.stack[self.stack_ptr] as char);
        },
        Token::LabelDef => {
          let name = self.get_identifier()?;
          if self.labels.contains_key(&name) {
            return Err(DSAsmError::InterpreterError(format!("Label '{}' already exists", &name)).into());
          }
          self.labels.insert(name, self.base.get_peek());
        },
        Token::Jmp => {
          let name = self.get_identifier()?;
          self.label_must_not_exist(&name)?;
          self.base.set_peek(self.labels[&name]);
        },
        Token::Jze => {
          let name = self.get_identifier()?;
          self.label_must_not_exist(&name)?;
          if self.stack[self.stack_ptr] == 0 {
            self.base.set_peek(self.labels[&name]);
          }
        },
        Token::Exclamation => {
          if self.stack[self.stack_ptr] == 0 {
            self.stack[self.stack_ptr] = 1;
          } else {
            self.stack[self.stack_ptr] = 0;
          };
        },
        Token::Star => {
          let a = self.stack[self.stack_ptr];
          let b = self.stack[(self.stack_ptr + 1) % Interpreter::STACK_SIZE];
          self.stack[self.stack_ptr] = a * b;
        },
        Token::Slash => {
          let a = self.stack[self.stack_ptr];
          let bi = (self.stack_ptr + 1) % Interpreter::STACK_SIZE;
          let b = self.stack[bi];
          self.stack[self.stack_ptr] = a / b;
          self.stack[bi] = a % b
        },
        t => {
          return Err(DSAsmError::InterpreterError(format!("Unexpected Token '{}'", t)).into());
        }
      }
    }

    Ok(())
  }
}
