use rusqlite::Error;
use crate::db::conn::create_connections;

pub fn search_link_original(link: &String) -> Result<(), Error> {
  let conn = create_connections()?;

  let mut stmt = conn.prepare(
    "SELECT link, hashed_link FROM links WHERE link=?1"
  )?;

  let links_iter = stmt.query_map(
    &[link],
    |row| Ok((row.get(0)?, row.get(1)?))
  )?;

  for link_info in links_iter {
    let (original_link, hashed_link): (String, String) = link_info?;
    println!("Original Link: {}", original_link);
    println!("Shortened Link: {}", hashed_link);
  }

  Ok(())
}

pub fn search_link_shortened(link: &String) -> Result<(), Error> {
  let conn = create_connections()?;

  let mut stmt = conn.prepare(
    "SELECT link, hashed_link FROM links WHERE hashed_link=?1"
  )?;

  let links_iter = stmt.query_map(
    &[link],
    |row| Ok((row.get(0)?, row.get(1)?))
  )?;

  for link_info in links_iter {
    let (original_link, hashed_link): (String, String) = link_info?;
    println!("Original Link: {}", original_link);
    println!("Shortened Link: {}", hashed_link);
  }

  Ok(())
}
