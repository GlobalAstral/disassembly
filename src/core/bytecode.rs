use std::{fmt::Display, process::Output};

use crate::core::{error::DSAsmError, processor::Processor, tokenizer::Token};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum Instruction {
  MoveStack(u8),
  Increment(u8),
  Decrement(u8),
  UserInput,
  Print,
  Label(String),
  Jump(String),
  JumpZero(String),
  JumpNotZero(String),
  Invert,
  Multiply,
  Divide,
  Clear,
  Dereference(u8),
  Goto(u8),
  Skip,
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

  fn get_literal(&mut self) -> Result<u8, DSAsmError> {
    match self.base.consume() {
      Token::Literal(lit) => Ok(lit),
      t => {
        Err(DSAsmError::ConverterError(format!("Expected literal instead of '{}'", t)).into())
      }
    }
  }

  pub fn convert(&mut self) -> Result<Vec<Instruction>, DSAsmError> {
    let mut output: Vec<Instruction> = Vec::new();

    while self.base.has_peek() {
      match self.base.consume() {
        Token::LabelDef => {
          output.push(Instruction::Label(self.get_identifier()?));
        },
        _ => { }
      }
    }
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
          let mut count: u8 = 1;
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
          let mut count: u8 = 1;
          while self.base.tryconsume(Token::Minus) {
            count += 1;
          };
          Instruction::Decrement(count)
        },
        Token::Comma => Instruction::UserInput,
        Token::Dot => Instruction::Print,
        Token::LabelDef => {
          self.base.consume();
          Instruction::Skip
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
        Token::Star => Instruction::Multiply,
        Token::Slash => Instruction::Divide,
        Token::Tilde => Instruction::Clear,
        Token::OpenSquare => {
          let val = self.get_literal()?;
          self.base.require(Token::CloseSquare).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ConverterError(format!("{}", e))))?;
          Instruction::Dereference(val)
        },
        t => {
          return Err(DSAsmError::ConverterError(format!("Unexpected Token '{}'", t)).into());
        }
      };
      if ins != Instruction::Skip {
        output.push(ins);
      }
    }
    Ok(output)
  }
}
