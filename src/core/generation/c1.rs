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
    let id = generate_id();
    let temp: &str = &format!("__{}_move", id);
    let skip: &str = &format!("__{}_skip_move", id);
    self.goto(src);
    self.jze(skip);

    self.create_label(temp);
    self.goto(dst);
    self.add(1);
    self.goto(src);
    self.sub(1);
    self.jnze(temp);
    self.create_label(skip);
  }

  pub fn copy(&mut self, dst: MemoryUnit, src: MemoryUnit) -> Result<(), DSAsmError> {
    self.clear(dst);
    let id = generate_id();
    let skip: &str = &format!("__{}_skip_copy", id);
    self.goto(src);
    self.jze(skip);

    let temporary = self.alloc_temp()?;
    self.clear(temporary);
    
    let temp: &str = &format!("__{}_copy", id);
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

    self.create_label(skip);

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

  pub fn mem_add(&mut self, dst: MemoryUnit, src: MemoryUnit) {
    let id = generate_id();
    let temp: &str = &format!("__{}_mem_add", id);
    let skip: &str = &format!("__{}_skip_mem_add", id);
    self.goto(src);
    self.jze(skip);
    self.create_label(temp);
    self.goto(dst);
    self.add(1);
    self.goto(src);
    self.sub(1);
    self.jnze(temp);
    self.create_label(skip);
  }

  pub fn mem_sub(&mut self, dst: MemoryUnit, src: MemoryUnit) {
    let id = generate_id();
    let temp: &str = &format!("__{}_mem_sub", id);
    let skip: &str = &format!("__{}_skip_mem_sub", id);
    self.goto(src);
    self.jze(skip);
    self.create_label(temp);
    self.goto(dst);
    self.jze(skip);
    self.sub(1);
    self.goto(src);
    self.sub(1);
    self.goto(src);
    self.jnze(temp);
    self.create_label(skip);
  }

  pub fn cmp(&mut self, l: MemoryUnit, r: MemoryUnit) {
    self.goto(l);
    self.push(Token::Apostrophe);
    self.push(Token::Literal(r));
  }

  pub fn or(&mut self, l: MemoryUnit, r: MemoryUnit) {
    self.goto(l);
    self.push(Token::Or);
    self.push(Token::Literal(r));
  }

  pub fn reduce(&mut self) {
    self.push(Token::Exclamation);
    self.push(Token::Exclamation);
  }
}
