use colored::*;

pub fn help_center() {
  println!("{}", "======================================================================".cyan());
  println!("{}", "                  📘 The Book: CutLink 📘".cyan());
  println!("{}", "======================================================================".cyan());
  println!("{}", "                Welcome to CutLink Help Center".cyan());
  println!("{}", "----------------------------------------------------------------------".cyan());
  println!("{}", "                  🛠️ Available Commands:".bright_green());
  println!("{}", "  cvert [URL]              Shorten a URL".bright_green());
  println!("{}", "  show all                 List all shortened URLs".bright_green());
  println!("{}", "  search -ol/-sl [CODE]    Retrieve a shortened URL by code".bright_green());
  println!("{}", "  delete [CODE]            Delete a shortened URL by code".bright_green());
  println!("{}", "  help                     Show this help message".bright_green());
  println!("{}", "----------------------------------------------------------------------".cyan());
}