use crate::core::{error::DSAsmError, generation::{Cell, Generator}, parser::{Expr, Node}, tokenizer::Token};


impl Generator {
  pub fn generate_expr(&mut self, expr: &Expr) -> Result<(), DSAsmError> {

    match expr {
      Expr::Literal(l) => {
        self.add(*l);
      },
      Expr::Variable(id) => {
        let (ptr, _) = self.stack.iter().enumerate().find(|(_, cell)| cell.is_variable_of_id(*id)).unwrap();
        self.copy(self.pointer as u8, ptr as u8)?;
      },
      _ => {
        unimplemented!()
      }
    }

    Ok(())
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
        self.clear(cell);
        self.goto(cell);
        self.generate_expr(expr)?;
        self.stack[cell as usize] = Cell::Variable(*id);
      },
      Node::VarSet(id, expr) => {
        let (i, _) = self.stack.iter().enumerate().find(|(_, cell)| cell.is_variable_of_id(*id)).unwrap();
        self.clear(i as u8);
        self.goto(i as u8);
        self.generate_expr(expr)?;
      },

      _ => {
        unimplemented!()
      }
    }

    Ok(())
  }
}
