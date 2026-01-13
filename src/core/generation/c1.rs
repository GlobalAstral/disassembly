use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::{error::DSAsmError, generation::{Cell, Generator}, interpreter::Interpreter};

static CURR_LABEL: AtomicU64 = AtomicU64::new(0);
fn generate_id() -> u64 {
  CURR_LABEL.fetch_add(1, Ordering::Relaxed)
}

impl Generator {
  pub fn left(&mut self) {
    self.goto(self.pointer + 1);
  }
  pub fn right(&mut self) {
    self.goto(self.pointer - 1);
  }
  pub fn alloc(&mut self) -> Result<u8, DSAsmError> {
    if let Some((i, cell)) = self.stack.iter_mut().enumerate().find(|(_, cell)| cell.is_unused()) {
      *cell = Cell::Used;
      Ok(i as u8)
    } else {
      Err(DSAsmError::CompilerError("Not enough memory!".into()))
    }
  }

  pub fn alloc_temp(&mut self) -> Result<u8, DSAsmError> {
    if let Some((i, cell)) = self.stack.iter_mut().enumerate().find(|(_, cell)| cell.is_unused()) {
      *cell = Cell::Temporary;
      Ok(i as u8)
    } else {
      Err(DSAsmError::CompilerError("Not enough memory!".into()))
    }
  }

  pub fn free(&mut self, addr: u8) {
    if let Some(cell) = self.stack.iter_mut().nth(addr as usize) {
      *cell = Cell::Unused;
    }
  }
  pub fn clear(&mut self, loc: u8) {
    self.goto(loc);
    let lbl_id = generate_id();
    let skip: &str = &format!("__{}_skip_clear", lbl_id);
    self.jze(skip);
    let temp: &str = &format!("__{}_clear", lbl_id);
    self.create_label(temp);

    self.goto(loc);
    self.sub(1);
    self.jnze(temp);
    self.create_label(skip);
  }

  pub fn r#move(&mut self, dst: u8, src: u8) {
    self.clear(dst);
    let temp: &str = &format!("__{}_move", generate_id());
    self.create_label(temp);
    self.goto(dst);
    self.add(1);
    self.goto(src);
    self.sub(1);
    self.jnze(temp);
  }

  pub fn copy(&mut self, dst: u8, src: u8) -> Result<(), DSAsmError> {
    let temporary = self.alloc_temp()?;
    self.clear(temporary);
    self.clear(dst);
    
    let temp: &str = &format!("__{}_copy", generate_id());
    self.create_label(temp);
    
    self.goto(dst);
    self.add(1);
    
    self.goto(temporary);
    self.add(1);
    
    self.goto(src);
    self.sub(1);
    
    self.jnze(temp);
    
    self.r#move(src, temporary);
    
    self.free(temporary);
    self.goto(dst);
    Ok(())
  }
}
