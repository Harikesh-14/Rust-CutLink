mod db;

mod operations {
  pub mod help;
  pub mod convert;
  pub mod display;
  pub mod delete;
  pub mod clear;
  pub mod search;
  pub mod list;
}

use std::error::Error;
use colored::*;
use operations::{help, convert, display, delete, clear, search, list};

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
    "search" => {
      let flag = config.arg2.as_str();
      match flag {
        "-ol" => {
          search::search_link_original(&config.arg3)?;
        }
        "-sl" => {
          search::search_link_shortened(&config.arg3)?;
        }
        _ => {
          println!("Invalid flag: {}", flag);
          println!("Usage:");
          println!("  search -ol <link>  (for finding the original link)");
          println!("  search -sl <link>  (for finding the shortened link)");
        }
      }
    }
    "delete" => {
      let code = config.arg2;
      let code = code.trim().parse().expect("Expected an integer");
      delete::delete_link(code)?;
    }
    "clear" => {
      clear::clear_table()?;
    }
    "list" => {
      let flag = config.arg2.as_str();

      match flag {
        "-ol" => {
          list::list_link_original(&config.arg3)?;
        }
        "-sl" => {
          list::list_link_shortened(&config.arg3)?;
        }
        _ => {
          println!("Invalid flag: {}", flag);
          println!("Usage:");
          println!("  list -ol <link>  (for finding the original link)");
          println!("  list -sl <link>  (for finding the shortened link)");
        }
      }
    }
    _ => {
      println!("Unknown operation")
    }
  }

  Ok(())
}