use crate::core::{error::DSAsmError, generation::{Cell, Generator}, interpreter::MemoryUnit, parser::{BinaryOperator, Expr, Node, UnaryOperator}, tokenizer::Token};


impl Generator {
  pub fn generate_expr(&mut self, expr: &Expr) -> Result<MemoryUnit, DSAsmError> {

    match expr {
      Expr::Literal(l) => {
        let cell = self.alloc_temp()?;
        self.clear(cell);
        self.goto(cell);
        self.add(*l);
        Ok(cell)
      },
      Expr::Variable(id) => {
        let cell = self.alloc_temp()?;
        let (ptr, _) = self.stack.iter().enumerate().find(|(_, cell)| cell.is_variable_of_id(*id)).unwrap();
        self.copy(cell, ptr as MemoryUnit)?;
        Ok(cell)
      },
      Expr::UserInput => {
        let cell = self.alloc_temp()?;
        self.clear(cell);
        self.goto(cell);
        self.getchar();
        Ok(cell)
      },
      Expr::Reference(ex) => {
        match ex.as_ref() {
          Expr::Variable(id) => {
            let cell = self.alloc_temp()?;
            let (ptr, _) = self.stack.iter().enumerate().find(|(_, cell)| cell.is_variable_of_id(*id)).unwrap();
            self.clear(cell);
            self.goto(cell);
            self.add(ptr as MemoryUnit);
            Ok(cell)
          },
          _ => {
            return Err(DSAsmError::CompilerError(format!("Cannot reference an unstable address {}", ex)));
          }
        }
      },
      Expr::Dereference(ex) => {
        let ex = self.generate_expr(&ex)?;
        let cell = self.alloc_temp()?;
        self.clear(cell);
        self.goto(cell);
        self.push(Token::OpenSquare);
        self.push(Token::Literal(ex));
        self.push(Token::CloseSquare);
        Ok(cell)
      },
      Expr::Unary(unary) => {
        let expr = self.generate_expr(&unary.right)?;
        self.goto(expr);
        let result = match unary.operator {
          UnaryOperator::Not => {
            self.invert();
            expr
          },
          UnaryOperator::Bnot => {
            let temp = self.alloc_temp()?;
            self.clear(temp);
            self.add(MemoryUnit::MAX);
            self.mem_sub(temp, expr);
            temp
          },
          UnaryOperator::Negate => {
            let temp = self.alloc_temp()?;
            self.clear(temp);
            self.add(MemoryUnit::MAX);
            self.mem_sub(temp, expr);
            self.goto(temp);
            self.add(1);
            temp
          }
        };
        Ok(result)
      },
      Expr::Binary(bin) => {
        let left = self.generate_expr(&bin.left)?;
        let right = self.generate_expr(&bin.right)?;
        let result = match bin.operator {
          BinaryOperator::Add => {
            self.mem_add(left, right);
            left
          },
          BinaryOperator::Sub => {
            self.mem_sub(left, right);
            left
          },
          BinaryOperator::Mult => {
            self.goto(left);
            self.mul(right);
            left
          },
          BinaryOperator::Div => {
            self.goto(left);
            self.div(right);
            left
          },
          BinaryOperator::Modulus => {
            self.goto(left);
            self.div(right);
            right
          },
          BinaryOperator::Equals => {
            self.mem_sub(left, right);
            self.goto(left);
            self.invert();
            left
          },
          BinaryOperator::NotEquals => {
            self.mem_sub(left, right);
            self.goto(left);
            self.reduce();
            left
          },
          BinaryOperator::Greater => {
            self.cmp(left, right);
            self.goto(left);
            self.sub(1);
            self.invert();
            left
          },
          BinaryOperator::Less => {
            let ltemp = self.alloc_temp()?;
            let rtemp = self.alloc_temp()?;
            self.copy(ltemp, left)?;
            self.copy(rtemp, right)?;
            self.mem_sub(ltemp, rtemp);
            self.cmp(left, right);
            self.goto(left);
            self.sub(2);
            self.invert();
            self.mul(ltemp);
            left
          },
          BinaryOperator::Lessequ => {
            self.cmp(left, right);
            self.goto(left);
            self.sub(2);
            self.invert();
            left
          },
          BinaryOperator::Grequ => {
            let ltemp = self.alloc_temp()?;
            let rtemp = self.alloc_temp()?;
            self.copy(ltemp, left)?;
            self.copy(rtemp, right)?;
            self.mem_sub(ltemp, rtemp);
            self.goto(ltemp);
            self.invert();
            self.cmp(left, right);
            self.goto(left);
            self.sub(1);
            self.invert();
            self.mem_add(left, ltemp);
            self.goto(left);
            self.reduce();
            left
          },
          BinaryOperator::ShiftL => {
            self.goto(left);
            self.push(Token::LeftAngle);
            self.push(Token::LeftAngle);
            self.push(Token::Literal(right));
            left
          },
          BinaryOperator::ShiftR => {
            self.goto(left);
            self.push(Token::RightAngle);
            self.push(Token::RightAngle);
            self.push(Token::Literal(right));
            left
          },
          BinaryOperator::Band => {
            let not_left = self.alloc_temp()?;
            self.clear(not_left);
            self.add(MemoryUnit::MAX);
            self.mem_sub(not_left, left);

            let not_right = self.alloc_temp()?;
            self.clear(not_right);
            self.add(MemoryUnit::MAX);
            self.mem_sub(not_right, right);
            self.or(not_left, not_right);
            
            let result = self.alloc_temp()?;
            self.clear(result);
            self.add(MemoryUnit::MAX);
            self.mem_sub(result, not_left);
            result
          },
          BinaryOperator::Bor => {
            self.or(left, right);
            left
          },
          BinaryOperator::And => {
            self.goto(right);
            self.reduce();
            self.goto(left);
            self.reduce();
            self.push(Token::Star);
            self.push(Token::Literal(right));
            left
          },
          BinaryOperator::Or => {
            self.mem_add(left, right);
            self.goto(left);
            self.reduce();
            left
          },
        };
        Ok(result)
      },
      Expr::MethodCall(_id, _params) => {
        unimplemented!()
      }
    }

  }

  pub fn generate(&mut self, node: &Node) -> Result<(), DSAsmError> {

    match node {
      Node::Scope(scope) => {
        for ele in scope {
          self.generate(ele)?;
        }
      },
      Node::VarDecl(id, expr) => {
        let cell = self.alloc()?;
        let ex = self.generate_expr(expr)?;
        self.copy(cell, ex)?;
        self.stack[cell as usize] = Cell::Variable(*id);
        self.free_temps();
      },
      Node::VarSet(id, expr) => {
        let (i, _) = self.stack.iter().enumerate().find(|(_, cell)| cell.is_variable_of_id(*id)).unwrap();
        let ex = self.generate_expr(expr)?;
        self.copy(i as MemoryUnit, ex)?;
        self.free_temps();
      },

      _ => {
        unimplemented!()
      }
    }

    Ok(())
  }
}
