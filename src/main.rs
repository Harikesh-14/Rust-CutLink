use colored::*;
use std::env;
use cutlink::Config;

fn main() {
    println!();
    println!("{}", "======================================================================".cyan());
    println!("{}", "                      Welcome to CutLink ✂️".cyan());
    println!("{}", "======================================================================".cyan());

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = cutlink::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
