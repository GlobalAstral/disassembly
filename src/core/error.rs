use std::fmt::Display;

#[derive(Debug)]
pub enum DSAsmError {
  ProcessorError(String),
  TokenizerError(String),
  InterpreterError(String),
  ArgumentError(String),
  GenericError(String),
  FileError(String),
  ParserError(String),
}

impl Display for DSAsmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DSAsmError::ProcessorError(e) => write!(f, "ProcessorError: {}", e),
      DSAsmError::TokenizerError(e) => write!(f, "TokenizerError: {}", e),
      DSAsmError::InterpreterError(e) => write!(f, "InterpreterError: {}", e),
      DSAsmError::ArgumentError(e) => write!(f, "ArgumentError: {}", e),
      DSAsmError::GenericError(e) => write!(f, "GenericError: {}", e),
      DSAsmError::FileError(e) => write!(f, "FileError: {}", e),
      DSAsmError::ParserError(e) => write!(f, "ParserError: {}", e),
    }
  }
}

impl std::error::Error for DSAsmError { }

impl<V, E: Display> From<Result<V, E>>  for DSAsmError {
  fn from(value: Result<V, E>) -> Self {
    DSAsmError::GenericError(format!("{}", value.err().expect("Result is not error")))
  }
}
