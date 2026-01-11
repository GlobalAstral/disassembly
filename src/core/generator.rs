use crate::core::interpreter::Interpreter;



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
