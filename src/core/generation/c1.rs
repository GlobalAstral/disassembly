use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::{error::DSAsmError, generation::{Cell, Generator}, interpreter::{Interpreter, MemoryUnit}, tokenizer::Token};

static CURR_LABEL: AtomicU64 = AtomicU64::new(0);
fn generate_id() -> u64 {
  CURR_LABEL.fetch_add(1, Ordering::Relaxed)
}

impl Generator {
  pub fn alloc(&mut self) -> Result<MemoryUnit, DSAsmError> {
    if let Some((i, cell)) = self.stack.iter_mut().enumerate().find(|(_, cell)| cell.is_unused()) {
      *cell = Cell::Used;
      Ok(i as MemoryUnit)
    } else {
      Err(DSAsmError::CompilerError("Not enough memory!".into()))
    }
  }

  pub fn alloc_temp(&mut self) -> Result<MemoryUnit, DSAsmError> {
    if let Some((i, cell)) = self.stack.iter_mut().enumerate().find(|(_, cell)| cell.is_unused()) {
      *cell = Cell::Temporary;
      Ok(i as MemoryUnit)
    } else {
      Err(DSAsmError::CompilerError("Not enough memory!".into()))
    }
  }

  pub fn alloc_param(&mut self, param_id: u64) -> Result<MemoryUnit, DSAsmError> {
    if let Some((i, cell)) = self.stack.iter_mut().enumerate().find(|(_, cell)| cell.is_unused()) {
      *cell = Cell::Parameter(param_id);
      Ok(i as MemoryUnit)
    } else {
      Err(DSAsmError::CompilerError("Not enough memory!".into()))
    }
  }

  pub fn free(&mut self, addr: MemoryUnit) {
    if let Some((addr, cell)) = self.stack.iter_mut().enumerate().nth(addr as usize) {
      *cell = Cell::Unused;
      self.clear(addr as MemoryUnit);
    }
  }
  pub fn clear(&mut self, loc: MemoryUnit) {
    self.goto(loc);
    self.push(Token::Tilde);
  }

  pub fn r#move(&mut self, dst: MemoryUnit, src: MemoryUnit) {
    self.clear(dst);
    let temp: &str = &format!("__{}_move", generate_id());
    self.create_label(temp);
    self.goto(dst);
    self.add(1);
    self.goto(src);
    self.sub(1);
    self.jnze(temp);
  }

  pub fn copy(&mut self, dst: MemoryUnit, src: MemoryUnit) -> Result<(), DSAsmError> {
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

  pub fn free_temps(&mut self) {
    let temp_indices: Vec<usize> = self.stack.iter()
        .enumerate()
        .filter_map(|(i, cell)| if cell.is_temp() { Some(i) } else { None })
        .rev()
        .collect();
    temp_indices.iter().for_each(|addr| {
      self.free(*addr as MemoryUnit);
    });
  }
}
