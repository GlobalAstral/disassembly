use std::{fmt::Display};

use crate::core::{error::DSAsmError, interpreter::MemoryUnit, processor::Processor, tokenizer::Token};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum Instruction {
  MoveStack(MemoryUnit),
  Increment(MemoryUnit),
  Decrement(MemoryUnit),
  UserInput,
  Print,
  Label(String),
  Jump(String),
  JumpZero(String),
  JumpNotZero(String),
  Invert,
  Multiply(MemoryUnit),
  Divide(MemoryUnit),
  Clear,
  Dereference(MemoryUnit),
  Goto(MemoryUnit),
  Compare(MemoryUnit),
  ShiftL(MemoryUnit),
  ShiftR(MemoryUnit),
  Or(MemoryUnit),
  #[default]
  Invalid
}

impl Display for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct BytecodeConverter {
  base: Processor<Token>
}

impl BytecodeConverter {
  pub fn new(i: Vec<Token>) -> BytecodeConverter {
    BytecodeConverter { base: Processor::new(i) }
  }

  fn get_identifier(&mut self) -> Result<String, DSAsmError> {
    match self.base.consume() {
      Token::Identifier(id) => Ok(id),
      t => {
        Err(DSAsmError::ConverterError(format!("Expected Identifier instead of '{}'", t)).into())
      }
    }
  }

  fn get_literal(&mut self) -> Result<MemoryUnit, DSAsmError> {
    match self.base.consume() {
      Token::Literal(lit) => Ok(lit),
      t => {
        Err(DSAsmError::ConverterError(format!("Expected literal instead of '{}'", t)).into())
      }
    }
  }

  pub fn convert(&mut self) -> Result<Vec<Instruction>, DSAsmError> {
    let mut output: Vec<Instruction> = Vec::new();
    self.base.set_peek(0);
    while self.base.has_peek() {
      let ins: Instruction = match self.base.consume() {
        Token::Caret => {
          match self.base.consume() {
            Token::Literal(lit) => {
              Instruction::MoveStack(lit)
            },
            t => {
              return Err(DSAsmError::ConverterError(format!("Expected literal instead of {}", t)).into())
            }
          }
        },
        Token::Plus => {
          let mut count: MemoryUnit = 1;
          while self.base.tryconsume(Token::Plus) {
            count += 1;
          };
          Instruction::Increment(count)
        },
        Token::Minus if self.base.tryconsume(Token::RightAngle) => {
          match self.base.consume() {
            Token::Literal(lit) => {
              Instruction::Goto(lit)
            },
            t => {
              return Err(DSAsmError::ConverterError(format!("Expected literal instead of {}", t)).into())
            }
          }
        },
        Token::Minus => {
          let mut count: MemoryUnit = 1;
          while self.base.tryconsume(Token::Minus) {
            count += 1;
          };
          Instruction::Decrement(count)
        },
        Token::Comma => Instruction::UserInput,
        Token::Dot => Instruction::Print,
        Token::LabelDef => {
          Instruction::Label(self.get_identifier()?)
        },
        Token::Jmp => {
          Instruction::Jump(self.get_identifier()?)
        },
        Token::Jze => {
          Instruction::JumpZero(self.get_identifier()?)
        },
        Token::Jnze => {
          Instruction::JumpNotZero(self.get_identifier()?)
        },
        Token::Exclamation => Instruction::Invert,
        Token::Star => Instruction::Multiply(self.get_literal()?),
        Token::Slash => Instruction::Divide(self.get_literal()?),
        Token::Tilde => Instruction::Clear,
        Token::OpenSquare => {
          let val = self.get_literal()?;
          self.base.require(Token::CloseSquare).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ConverterError(format!("{}", e))))?;
          Instruction::Dereference(val)
        },
        Token::Apostrophe => Instruction::Compare(self.get_literal()?),
        Token::LeftAngle if self.base.tryconsume(Token::LeftAngle) => Instruction::ShiftL(self.get_literal()?),
        Token::RightAngle if self.base.tryconsume(Token::RightAngle) => Instruction::ShiftR(self.get_literal()?),
        Token::Or => Instruction::Or(self.get_literal()?),
        t => {
          return Err(DSAsmError::ConverterError(format!("Unexpected Token '{}'", t)).into());
        }
      };
      output.push(ins);
    }
    Ok(output)
  }
}
