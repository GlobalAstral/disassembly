use std::{env::args, error::Error, fs::{self, File}, io::Read};

use crate::core::{error::DSAsmError, generation::Generator, interpreter::{self, Interpreter}, parser::Parser, tokenizer::Tokenizer};


mod core;

fn main() -> Result<(), DSAsmError>{

  let args: Vec<String> = args().collect();

  let fname: &str = if args.len() > 1 {
    args.get(1).unwrap()
  } else {
    return Err(DSAsmError::ArgumentError("Invalid command line arguments".into()).into());
  };

  let raw = args.contains(&"-raw".to_string());
  let debug = args.contains(&"-debug".to_string());
  let content: String = fs::read_to_string(fname).map_err(|e| Err::<File, DSAsmError>(DSAsmError::FileError(format!("{}", e))))?;

  let mut tokenizer: Tokenizer = Tokenizer::new(content.chars().collect());
  let tokens = if raw {
    tokenizer.tokenize()
  } else {
    let tokens = tokenizer.tokenize()?;
    if debug {
      println!("\nTOKENS:");
      tokens.iter().for_each(|t| println!("{}", t));
    }
    let mut parser = Parser::new(tokens);
    let nodes = parser.parse_all()?;
    if debug {
      println!("\nNODES:");
      nodes.iter().for_each(|e| println!("{}", e));
    }
    let mut generator = Generator::new(nodes);
    let ret = generator.generate_all();
    println!("\nMEMORY CELLS:");
    generator.print_memory();
    ret
  }?;

  if debug {
    println!("\nGENERATED:");
    tokens.iter().for_each(|t| println!("{}", t));
  }

  let mut interpreter: Interpreter = Interpreter::new(tokens);

  interpreter.interpret()?;

  if debug {
    println!("\nAddress - Value:");
    interpreter.print_memory();
  }

  Ok(())
}
