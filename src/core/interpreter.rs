use std::{collections::HashMap, io::{Read, Write, stdin, stdout}};

use crate::core::{bytecode::Instruction, error::DSAsmError, processor::{Processor, ProcessorInput}};

pub type MemoryUnit = u16;

pub struct Interpreter {
  base: Processor<Instruction>,
  stack: [MemoryUnit; Interpreter::STACK_SIZE],
  stack_ptr: usize,
  labels: HashMap<String, usize>
}

impl ProcessorInput for Instruction { }

impl Interpreter {
  const MAX_STACK_SIZE: usize = 1024;
  pub const STACK_SIZE: usize = {
    let max = MemoryUnit::MAX as usize;
    if max > Interpreter::MAX_STACK_SIZE { Interpreter::MAX_STACK_SIZE } else { max }
  };
  pub fn new(content: Vec<Instruction>) -> Interpreter {
    Interpreter { 
      base: Processor::new(content),
      stack: [0; Interpreter::STACK_SIZE],
      stack_ptr: 0,
      labels: HashMap::new()
    }
  }

  pub fn print_memory(&self) {
    let side: usize = (Interpreter::STACK_SIZE as f32).sqrt() as usize;
    let maximum: &MemoryUnit = self.stack.iter().max().unwrap();
    let digits: usize = maximum.to_string().len();
    let hex_size_len: usize = std::mem::size_of::<MemoryUnit>() * 2 + 2;
    self.stack.chunks(side).enumerate().for_each(|(addr, value)| {
      let temp: String = value.iter().map(|u| format!("{:0digits$}", u)).collect::<Vec<String>>().join(" | ") + " |";
      println!("{:#0hex_size_len$X} | {}", addr * value.len(), temp);
    });
  }

  fn label_must_exist(&self, name: &str) -> Result<(), DSAsmError> {
    if !self.labels.contains_key(&name.to_string()) {
      return Err(DSAsmError::InterpreterError(format!("Label '{}' does not exists", &name)).into());
    }
    Ok(())
  }

  pub fn interpret(&mut self) -> Result<(), DSAsmError> {
    while self.base.has_peek() {
      match self.base.consume() {
        Instruction::Label(name) => {
          if self.labels.contains_key(&name) {
            return Err(DSAsmError::InterpreterError(format!("Label '{}' already exists", &name)).into());
          }
          self.labels.insert(name, self.base.get_peek());
        },
        _ => { }
      }
    }
    self.base.set_peek(0);
    while self.base.has_peek() {
      match self.base.consume() {
        Instruction::MoveStack(addr) => {
          if addr as usize >= Interpreter::STACK_SIZE {
            return Err(DSAsmError::InterpreterError(format!("Invalid address {}", addr)).into())
          }
          self.stack_ptr = addr as usize;
        },
        Instruction::Increment(amount) => {
          let tmp: MemoryUnit = self.stack[self.stack_ptr];
          self.stack[self.stack_ptr] = tmp.wrapping_add(amount);
        },
        Instruction::Decrement(amount) => {
          let tmp: MemoryUnit = self.stack[self.stack_ptr];
          self.stack[self.stack_ptr] = tmp.wrapping_sub(amount);
        },
        Instruction::UserInput => {
          stdout().flush().ok();
          let mut buf: [u8; 1] = [0];
          stdin().read_exact(&mut buf).expect("Cannot read user input");
          self.stack[self.stack_ptr] = buf[0] as MemoryUnit;
        },
        Instruction::Print => {
          print!("{}", (self.stack[self.stack_ptr] as u8) as char);
        },
        Instruction::Label(_) => { },
        Instruction::Jump(name) => {
          self.label_must_exist(&name)?;
          self.base.set_peek(self.labels[&name]);
        },
        Instruction::JumpZero(name) => {
          self.label_must_exist(&name)?;
          if self.stack[self.stack_ptr] == 0 {
            self.base.set_peek(self.labels[&name]);
          }
        },
        Instruction::JumpNotZero(name) => {
          self.label_must_exist(&name)?;
          if self.stack[self.stack_ptr] != 0 {
            self.base.set_peek(self.labels[&name]);
          }
        },
        Instruction::Invert => {
          if self.stack[self.stack_ptr] == 0 {
            self.stack[self.stack_ptr] = 1;
          } else {
            self.stack[self.stack_ptr] = 0;
          };
        },
        Instruction::Multiply(addr) => {
          let a = self.stack[self.stack_ptr];
          let b = self.stack[addr as usize];
          self.stack[self.stack_ptr] = a * b;
        },
        Instruction::Divide(addr) => {
          let a = self.stack[self.stack_ptr];
          let b = self.stack[addr as usize];
          self.stack[self.stack_ptr] = a / b;
          self.stack[addr as usize] = a % b
        },
        Instruction::Clear => {
          self.stack[self.stack_ptr] = 0;
        },
        Instruction::Dereference(addr) => {
          let addr = self.stack[addr as usize];
          self.stack[self.stack_ptr] = self.stack[addr as usize];
        },
        Instruction::Goto(ip) => {
          self.base.set_peek(ip as usize);
        },
        Instruction::Compare(addr) => {
          let left = self.stack[self.stack_ptr];
          let right = self.stack[addr as usize];
          let value = if left > right {1} else {2};
          self.stack[self.stack_ptr] = value;
            
        },
        Instruction::ShiftL(addr) => {
          let temp = self.stack[addr as usize] % (std::mem::size_of::<MemoryUnit>() * 8) as MemoryUnit;
          self.stack[self.stack_ptr] = self.stack[self.stack_ptr] << temp;
        },
        Instruction::ShiftR(addr) => {
          let temp = self.stack[addr as usize] % (std::mem::size_of::<MemoryUnit>() * 8) as MemoryUnit;
          self.stack[self.stack_ptr] = self.stack[self.stack_ptr] >> temp;
        },
        Instruction::Or(addr) => {
          self.stack[self.stack_ptr] = self.stack[self.stack_ptr] | self.stack[addr as usize];
        },
        t => {
          return Err(DSAsmError::InterpreterError(format!("Unexpected Instruction '{}'", t)).into());
        }
      }
    }

    Ok(())
  }
}
