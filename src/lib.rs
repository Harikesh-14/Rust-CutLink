mod db;

mod operations {
  pub mod help;
  pub mod convert;
  pub mod display;
  // pub mod unhash;
}

use std::error::Error;
use colored::*;
use operations::{help, convert, display};

pub struct Config {
  pub operation: String,
  pub arg2: String,
  pub arg3: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 1 {
      println!("{}", "Use at least 1 arguments".red())
    }

    let operation = args[1].clone();
    let arg2 = args.get(2).cloned().unwrap_or_else(|| String::from("-a"));
    let arg3 = args.get(3).cloned().unwrap_or_else(|| String::from("-a"));

    Ok(Config { operation, arg2, arg3 })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  match config.operation.as_str() {
    "help" => {
      help::help_center();
    }
    "cvert" => {
      convert::convert_link(&config.arg2)?;
    }
    "show" => {
      match config.arg2.as_str() {
        "all" => {
          display::display_stored_links()?;
        }
        _ => {
          println!("You mean:");
          println!("`cargo run show all`");
          println!("\tOR");
          println!("`cutlink show all`")
        }
      }
    }
    _ => {
      println!("These are only the available operations: ");
      println!("1. Convert (Hashing)");
      println!("2. Show");
      println!("3. Unhash");
      println!("4. Delete");
      println!("5. Search");
    }
  }

  Ok(())
}