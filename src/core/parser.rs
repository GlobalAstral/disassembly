use std::{default, fmt::Display, sync::atomic::{AtomicU64, Ordering}};

use crate::core::{error::DSAsmError, interpreter::Interpreter, processor::Processor, tokenizer::Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
  pub name: String,
  pub id: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
      Self::Equals | Self::NotEquals => 5,
      Self::Greater | Self::Less | Self::Grequ | Self::Lessequ => 6,
      Self::ShiftL | Self::ShiftR => 7,
      Self::Add | Self::Sub => 8,
      Self::Mult | Self::Div | Self::Modulus => 9
    }
  }
}

impl BinaryOperator {
  fn to_str(self) -> &'static str {
    match self {
      BinaryOperator::Add => "+",
      BinaryOperator::And => "&&",
      BinaryOperator::Band => "&",
      BinaryOperator::Bor => "|",
      BinaryOperator::Bxor => "^",
      BinaryOperator::Div => "/",
      BinaryOperator::Equals => "==",
      BinaryOperator::Greater => ">",
      BinaryOperator::Grequ => ">=",
      BinaryOperator::Less => "<",
      BinaryOperator::Lessequ => "<=",
      BinaryOperator::Modulus => "%",
      BinaryOperator::Mult => "*",
      BinaryOperator::NotEquals => "!=",
      BinaryOperator::Or => "||",
      BinaryOperator::ShiftL => "<<",
      BinaryOperator::ShiftR => ">>",
      BinaryOperator::Sub => "-"
    }
  }
}

impl Display for BinaryOperator  {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_str())
  }
}

impl UnaryOperator  {
  fn to_str(self) -> &'static str {
    match self {
      Self::Bnot => "~",
      Self::Negate => "-",
      Self::Not => "!"
    }
  }
}

impl Display for UnaryOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_str())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnaryOperator {
  Negate, Not, Bnot
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary {
  left: Box<Expr>,
  right: Box<Expr>,
  operator: BinaryOperator 
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unary {
  right: Box<Expr>,
  operator: UnaryOperator
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
  Literal(u8),
  Variable(u64),
  UserInput,
  Reference(Box<Expr>),
  Dereference(Box<Expr>),
  MethodCall(u64, Vec<Expr>),
  Binary(Binary),
  Unary(Unary),
}

impl Display for Expr  {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Literal(lit) => write!(f, "{}", lit),
      Self::Variable(var) => write!(f, "${{#{}}}", var),
      Self::Dereference(e) => write!(f, "*{}", e),
      Self::Reference(e) => write!(f, "&{}", e),
      Self::Unary(un) => write!(f, "{}{}", un.operator, un.right),
      Self::Binary(bin) => write!(f, "{} {} {}", bin.left, bin.operator, bin.right),
      Self::UserInput => write!(f, "getchar"),
      Self::MethodCall(id, params) => write!(f, "${{#{}}}({})", id, { let mut tmp: Vec<String> = Vec::new(); params.iter().for_each(|e| tmp.push(format!("{}", e))); tmp.join(", ")})
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForLoop {
  var_name: String,
  start: Expr,
  condition: Expr,
  increment: Box<Node>,
  body: Box<Node>
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Node {
  Scope(Vec<Node>),
  VarDecl(u64, Expr),
  VarSet(u64, Expr),
  If(Expr, Box<Node>),
  While(Expr, Box<Node>),
  For(ForLoop),
  Putchar(Expr),
  MethodDecl(Method),
  Return(Expr),
  #[default]
  Invalid
}

impl Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Scope(scope) => {
        write!(f, "{{\n")?;
        scope.iter().for_each(|node| {let _ = write!(f, "\t{}", node);});
        write!(f, "}}")?;
      },
      Self::VarDecl(name, ex) => { write!(f, "let #{} = {}", name, ex)?; },
      Self::VarSet(name, ex) => { write!(f, "#{} = {}", name, ex)?; },
      Self::Putchar(ex) => { write!(f, "putchar({})", ex)?; },
      Self::While(ex, body) => { write!(f, "while ({}) {}", ex, body)?; },
      Self::If(ex, body) => { write!(f, "if ({}) {}", ex, body)?; },
      Self::For(forloop) => { write!(f, "for ({} = {}; {}; {}) {}", forloop.var_name, forloop.start, forloop.condition, forloop.increment, forloop.body)?; },
      Self::MethodDecl(method) => { write!(f, "method {}[{}]({}) {}", method.name, method.id, method.parameters.iter().map(|v| format!("{}#{}", v.name, v.id)).collect::<Vec<String>>().join(", "), method.body)?; },
      Self::Return(e) => { write!(f, "return {}", e)?; },
      Self::Invalid => { write!(f, "Invalid")?; }
    };
    Ok(())
  }
}

static CURRENT_ID: AtomicU64 = AtomicU64::new(0);
fn generate_id() -> u64 {
  CURRENT_ID.fetch_add(1, Ordering::Relaxed)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method {
  pub name: String,
  pub id: u64,
  pub parameters: Vec<Variable>,
  pub body: Box<Node>
}

pub struct Parser {
  base: Processor<Token>,
  vars: Vec<Variable>,
  methods: Vec<Method>,
}

impl Parser {
  pub fn new(input: Vec<Token>) -> Parser {
    Parser { base: Processor::new(input), vars: Vec::new(), methods: Vec::new() }
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
      Token::Identifier(ident) => {
        if self.base.tryconsume(Token::OpenParen) {
          let temp = self.methods.iter().find(|mtd| mtd.name == ident);
          if temp.is_none() {
            return Err(DSAsmError::ParserError(format!("Method '{}' does not exist", ident)).into())
          }
          let id = temp.map(|e| e.id).unwrap();
          let mut params: Vec<Expr> = Vec::new();
          self.require_until(Token::CloseParen, |this| {
            if params.len() > 0 {
              this.base.require(Token::Comma).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
            }
            params.push(this.parseExpr(false)?);
            Ok(())
          })?;
          Expr::MethodCall(id, params)
        } else if let Some(ah) = self.vars.iter().find(|var| var.name == ident) {
          Expr::Variable(ah.id)
        } else {
          return Err(DSAsmError::ParserError(format!("Variable '{}' does not exist", ident)).into())
        }
      },
      Token::Getchar => Expr::UserInput,
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

  fn do_until<F>(&mut self, cmp: Token, mut f: F) -> Result<bool, DSAsmError> where F: FnMut(&mut Self) -> Result<(), DSAsmError> {
    while self.base.has_peek() {
      if self.base.tryconsume(cmp.clone()) {
        return Ok(true)
      }
      f(self)?;
    };
    Ok(false)
  }

  fn require_until<F>(&mut self, cmp: Token, f: F) -> Result<(), DSAsmError> where F: FnMut(&mut Self) -> Result<(), DSAsmError> {
    if !self.do_until(cmp.clone(), f)? {
      return Err(DSAsmError::ParserError(format!("Expected '{}' instead", cmp)).into())
    }
    Ok(())
  }

  fn parse(&mut self) -> Result<Node, DSAsmError> {
    let node = match self.base.consume() {
      Token::OpenCurly => {
        let mut scope: Vec<Node> = Vec::new();
        let old = self.vars.clone();
        self.require_until(Token::CloseCurly, |proc| {
          scope.push(proc.parse()?);
          Ok(())
        })?;
        self.vars = old;
        Node::Scope(scope)
      },
      Token::Let => {
        let name = match self.base.consume() {
          Token::Identifier(s) => s,
          t => {
            return Err(DSAsmError::ParserError(format!("Unexpected '{}', expected identifier instead", t)).into());
          }
        };
        if self.vars.iter().find(|e| e.name == name).is_some() {
          return Err(DSAsmError::ParserError(format!("Variable '{}' already exists", name)).into());
        };
        let var: Variable = Variable { name: name.clone(), id: generate_id() };
        self.vars.push(var.clone());
        self.base.require(Token::Equals).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        Node::VarDecl(var.id, self.parseExpr(false)?)
      },
      Token::Identifier(name) => {
        let var = self.vars.iter().find(|e| e.name == name);
        if var.is_none() {
          return Err(DSAsmError::ParserError(format!("Variable '{}' does not exists", name)).into());
        };
        self.base.require(Token::Equals).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        Node::VarSet(var.unwrap().id, self.parseExpr(false)?)
      },
      Token::For => {
        self.base.require(Token::OpenParen).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        let name = match self.base.consume() {
          Token::Identifier(s) => s,
          t => {
            return Err(DSAsmError::ParserError(format!("Unexpected '{}', expected identifier instead", t)).into());
          }
        };
        self.base.require(Token::Equals).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        let old = self.vars.clone();
        self.vars.push(Variable { name: name.clone(), id: generate_id() });
        let start = self.parseExpr(false)?;
        self.base.require(Token::Semicolon).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        let cond = self.parseExpr(false)?;
        self.base.require(Token::Semicolon).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        let inc = self.parse()?;
        self.base.require(Token::CloseParen).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        let body = self.parse()?;
        self.vars = old;
        Node::For(ForLoop { var_name: name, start, condition: cond, increment: Box::new(inc), body: Box::new(body) })
      },
      Token::If => Node::If(self.parseExpr(false)?, Box::new(self.parse()?)),
      Token::While => Node::While(self.parseExpr(false)?, Box::new(self.parse()?)),
      Token::Putchar => Node::Putchar(self.parseExpr(false)?),
      Token::Return => Node::Return(self.parseExpr(false)?),

      t => {
        return Err(DSAsmError::ParserError(format!("Unexpected '{}'", t)).into());
      }
    };
    
    Ok(node)
  }
  pub fn parse_all(&mut self) -> Result<Vec<Node>, DSAsmError> {
    let mut nodes: Vec<Node> = Vec::new();
    while self.base.has_peek() {
      if self.base.tryconsume(Token::Method) {
        let name = match self.base.consume() {
          Token::Identifier(s) => s,
          t => {
            return Err(DSAsmError::ParserError(format!("Unexpected '{}', expected identifier instead", t)).into());
          }
        };
        self.base.require(Token::OpenParen).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
        let mut params: Vec<Variable> = Vec::new();
        let old = self.vars.clone();
        self.require_until(Token::CloseParen, |this| {
          if params.len() > 0 {
            this.base.require(Token::Comma).map_err(|e| Err::<(), DSAsmError>(DSAsmError::ParserError(format!("{}", e)).into()))?;
          }
          match this.base.consume() {
            Token::Identifier(s) => {
              let var: Variable = Variable {id: generate_id(), name: s.clone()};
              this.vars.push(var.clone());
              params.push(var);
              return Ok(())
            },
            t => {
              return Err(DSAsmError::ParserError(format!("Unexpected '{}', expected identifier instead", t)).into());
            }
          };
        })?;
        let node = self.parse()?;
        self.vars = old;
        let mtd: Method = Method { name, id: generate_id(), parameters: params, body: Box::new(node) };
        self.methods.push(mtd.clone());
        nodes.push(Node::MethodDecl(mtd));
      } else {
        nodes.push(self.parse()?);
      }
    }
    Ok(nodes)
  }
}
