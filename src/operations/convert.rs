use rand::{thread_rng, Rng};
use crate::db::conn::create_connections;
use rusqlite::Error;

fn generate_random_string() -> String {
  let symbols = "abcdefghijklmnopqrstuvwxyz0123456789";
  let size = symbols.len();

  let mut generated_string = String::new();

  for _ in 0..8 {
    let random_number = thread_rng().gen_range(0..size);
    generated_string.push(symbols.chars().nth(random_number).unwrap())
  }

  generated_string
}

pub fn convert_link(link: &String) -> Result<(), Error> {
  let conn = create_connections()?;
  let random_string = &generate_random_string();

  match conn.execute(
    "INSERT INTO links (link, hashed_link) VALUES (?1, ?2)",
    &[link, &random_string],
  ) {
    Ok(_) => {
      println!("Your shorted link is {}", random_string);
      Ok(())
    },
    Err(err) => {
      eprintln!("Error hashing the link: {}", err);
      Err(err)
    }
  }
}