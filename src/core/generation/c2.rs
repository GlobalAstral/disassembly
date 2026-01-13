use crate::core::{error::DSAsmError, generation::{Cell, Generator}, parser::{Expr, Node}, tokenizer::Token};


impl Generator {
  pub fn generate_expr(&mut self, expr: &Expr) -> Result<(), DSAsmError> {

    match expr {
      Expr::Literal(l) => {
        self.add(*l);
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
        self.goto(cell);
        self.clear();
        self.generate_expr(expr)?;
        self.stack[cell as usize] = Cell::Variable(*id);
      },
      Node::VarSet(id, expr) => {
        let (i, cell) = self.stack.iter().enumerate().find(|(i, cell)| cell.is_variable_of_id(*id)).unwrap();
        self.goto(i as u8);
        self.clear();
        self.generate_expr(expr)?;
      },
      _ => {
        unimplemented!()
      }
    }

    Ok(())
  }
}
