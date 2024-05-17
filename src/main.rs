use std::env;
use cutlink::Config;

fn main() {
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
