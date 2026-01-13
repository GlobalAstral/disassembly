use crate::core::{error::DSAsmError, generation::{Cell, Generator}, parser::{Expr, Node}, tokenizer::Token};


impl Generator {
  pub fn generate_expr(&mut self, expr: &Expr) -> Result<u8, DSAsmError> {

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
        self.copy(cell, ptr as u8)?;
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
            self.add(ptr as u8);
            Ok(cell)
          },
          _ => {
            return Err(DSAsmError::CompilerError(format!("Cannot reference an unstable address {}", ex)));
          }
        }
      },
      _ => {
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
        self.copy(i as u8, ex)?;
        self.free_temps();
      },

      _ => {
        unimplemented!()
      }
    }

    Ok(())
  }
}
