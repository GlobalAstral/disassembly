use crate::core::{error::DSAsmError, interpreter::Interpreter, processor::Processor, tokenizer::Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Cell {
  #[default]
  Unused,
  Used,
  Variable(u64),
  Temporary
}

pub type Stack = [Cell; Interpreter::STACK_SIZE];

const EMPTY_STACK: Stack = [Cell::Unused; Interpreter::STACK_SIZE];

struct Variable {
  pub name: String,
  pub id: u64,
  pub cell: usize
}
#[derive(Debug)]
enum BinaryOperator {
  Add, Sub, Mult, Div, Modulus, Equals, NotEquals, Greater, Less, Grequ, Lessequ, ShiftR, ShiftL, Band, Bor, Bxor, And, Or
}

impl BinaryOperator {
  pub fn precedence(&self) -> u64 {
    match *self {
      Self::Or => 0,
      Self::And => 1,
      Self::Bor => 2,
      Self::Bxor => 3,
      Self::Band => 4,
      Self::Equals | Self::NotEquals => 5, //*
      Self::Greater | Self::Less | Self::Grequ | Self::Lessequ => 6, //*
      Self::ShiftL | Self::ShiftR => 7, //*
      Self::Add | Self::Sub => 8, //*
      Self::Mult | Self::Div | Self::Modulus => 9 //*
    }
  }
}
#[derive(Debug)]
enum UnaryOperator {
  Negate, Not, Bnot
}
#[derive(Debug)]
struct Binary {
  left: Box<Expr>,
  right: Box<Expr>,
  operator: BinaryOperator 
}
#[derive(Debug)]
struct Unary {
  right: Box<Expr>,
  operator: UnaryOperator
}

#[derive(Debug)]
enum Expr {
  Literal(u8),
  Variable(u64),
  Reference(Box<Expr>),
  Dereference(Box<Expr>),
  Binary(Binary),
  Unary(Unary)
}

pub struct Parser {
  base: Processor<Token>,
  cells: Stack,
  vars: Vec<Variable>
}

impl Parser {
  pub fn new(input: Vec<Token>) -> Parser {
    Parser { base: Processor::new(input), cells: EMPTY_STACK, vars: Vec::new() }
  }

  fn parseOperator(&mut self) -> Option<BinaryOperator> {
    let old = self.base.get_peek();
    let tmp = match self.base.consume() {
      Token::Plus => BinaryOperator::Add,
      Token::Minus => BinaryOperator::Sub,
      Token::Star => BinaryOperator::Mult,
      Token::Slash => BinaryOperator::Div,
      Token::Percent => BinaryOperator::Modulus,
      Token::LeftAngle if self.base.tryconsume(Token::LeftAngle) => BinaryOperator::ShiftL,
      Token::RightAngle if self.base.tryconsume(Token::RightAngle) => BinaryOperator::ShiftR,
      Token::LeftAngle if self.base.tryconsume(Token::Equals) => BinaryOperator::Lessequ,
      Token::RightAngle if self.base.tryconsume(Token::Equals) => BinaryOperator::Grequ,
      Token::LeftAngle => BinaryOperator::Less,
      Token::RightAngle => BinaryOperator::Greater,
      Token::Equals if self.base.tryconsume(Token::Equals) => BinaryOperator::Equals,
      Token::Exclamation if self.base.tryconsume(Token::Equals) => BinaryOperator::NotEquals,
      Token::Ampersand if self.base.tryconsume(Token::Ampersand) => BinaryOperator::And,
      Token::Ampersand => BinaryOperator::Band,
      Token::Pipe if self.base.tryconsume(Token::Pipe) => BinaryOperator::Or,
      Token::Pipe => BinaryOperator::Bor,
      Token::Caret => BinaryOperator::Bxor,
      _ => {
        self.base.set_peek(old);
        return None
      }
    };
    Some(tmp)
  }

  fn parseExpr(&mut self, paren: bool) -> Result<Expr, DSAsmError> {
    let mut left: Expr = match self.base.consume() {
      Token::Literal(val) => Expr::Literal(val),
      Token::Identifier(ident) => 
      if let Some(ah) = self.vars.iter().find(|var| var.name == ident) {
        Expr::Variable(ah.id)
      } else {
        return Err(DSAsmError::ParserError(format!("Variable '{}' does not exist", ident)).into())
      },
      Token::Ampersand => Expr::Reference(Box::new(self.parseExpr(paren)?)),
      Token::Star => Expr::Dereference(Box::new(self.parseExpr(paren)?)),
      Token::Minus => Expr::Unary(Unary { right: Box::new(self.parseExpr(paren)?), operator: UnaryOperator::Negate }),
      Token::Exclamation => Expr::Unary(Unary { right: Box::new(self.parseExpr(paren)?), operator: UnaryOperator::Not }),
      Token::Tilde => Expr::Unary(Unary { right: Box::new(self.parseExpr(paren)?), operator: UnaryOperator::Bnot }),
      Token::OpenParen => self.parseExpr(true)?,
      t => {
        return Err(DSAsmError::ParserError(format!("Invalid Token '{}'", t)).into())
      }
    };
    let operator = self.parseOperator();

    if let Some(operator) = operator {
      let right = self.parseExpr(false)?;
      let new_expr = match right {
        Expr::Binary(bin) => {
          if operator.precedence() > bin.operator.precedence() {
            let tmp = Expr::Binary(Binary { left: Box::new(left), right: bin.left, operator });
            Expr::Binary(Binary { left: Box::new(tmp), right: bin.right, operator: bin.operator })
          } else {
            Expr::Binary(Binary { left: Box::new(left), right: Box::new(Expr::Binary(bin)), operator })
          }
        },
        _ => {
          Expr::Binary(Binary { left: Box::new(left), right: Box::new(right), operator })
        }
      };
      left = new_expr;
    };
    if paren {
      self.base.require(Token::CloseParen).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
    }
    return Ok(left);
  }

  fn parse(&mut self) -> Result<(), DSAsmError> {


    
    Ok(())
  }
  pub fn parse_all(&mut self) -> Result<(), DSAsmError> {
    while self.base.has_peek() {
      self.parse()?;
    }
    Ok(())
  }
}
