use std::{env::args, fs::{self, File}};

use crate::core::{bytecode::BytecodeConverter, error::DSAsmError, generation::Generator, interpreter::{Interpreter}, parser::Parser, tokenizer::Tokenizer};


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

  let mut converter: BytecodeConverter = BytecodeConverter::new(tokens);

  let bytecode = converter.convert()?;

  if debug {
    println!("\nBYTECODE:");
    bytecode.iter().for_each(|ins| println!("{}", ins));
  }

  let mut interpreter: Interpreter = Interpreter::new(bytecode);

  //TODO Methods | Calls and Declaration
  //TODO All the statements
  //TODO Dereference value assign => *(Expr) = Value
  //TODO Expression as statement
  //TODO Think about structs (maybe force them as pointers and add some kind of low level stuff to access nearby fields)
  //TODO Think about arrays (kinda like structs but change syntax. Maybe let them work the same but just different syntax)

  interpreter.interpret()?;

  if debug {
    println!("\nAddress - Value:");
    interpreter.print_memory();
  }

  Ok(())
}
