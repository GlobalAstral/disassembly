use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::{error::DSAsmError, generation::{Cell, Generator}, interpreter::Interpreter};

static CURR_LABEL: AtomicU64 = AtomicU64::new(0);
fn generate_id() -> u64 {
  CURR_LABEL.fetch_add(1, Ordering::Relaxed)
}

impl Generator {
  pub fn goto(&mut self, ptr: u8) {
    if (ptr as usize) < Interpreter::STACK_SIZE {
      let delta: i16 = (ptr as i16) - (self.pointer as i16);
      if delta < 0 {
        self.left((-delta) as u8);
      } else if delta > 0 {
        self.right(delta as u8);
      }
    }
  }

  pub fn alloc(&mut self) -> Result<u8, DSAsmError> {
    if let Some((i, cell)) = self.stack.iter_mut().enumerate().find(|(_, cell)| cell.is_unused()) {
      *cell = Cell::Used;
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
  pub fn clear(&mut self) {
    let temp: &str = &format!("__{}_clear", generate_id());
    self.create_label(temp);
    self.sub(1);
    self.jnze(temp);
  }
}
