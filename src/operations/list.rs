use rusqlite::{Error, params};
use crate::db::conn::create_connections;

struct LinkFormat {
  id: i32,
  original_link: String,
  hashed_link: String,
}

pub fn list_link_original(link: &String) -> Result<(), Error> {
  let conn = create_connections()?;
  let pattern = format!("%{}%", link);
  let mut stmt = conn.prepare(
    "SELECT link, hashed_link FROM links WHERE link LIKE ?1"
  )?;
  let link_iter = stmt.query_map(params![pattern], |row| {
    Ok(LinkFormat {
      id: row.get(0)?,
      original_link: row.get(1)?,
      hashed_link: row.get(2)?,
    })
  })?;

  println!("\n                          STORED LINKS\n");
  println!("======================================================================");
  println!("{:<4} | {:<40} | {:<20}", "ID", "Original Link", "Shortened Link");
  println!("----------------------------------------------------------------------");
  for i in link_iter {
    match i {
      Ok(link) => {
        let curr_id = link.id;
        let curr_original_link = crate::operations::display::format_original_link(&link.original_link);

        // Format hashed link
        let curr_hashed_link = crate::operations::display::format_link(&link.hashed_link);

        println!("{:<4} | {:<40} | {}", curr_id, curr_original_link, curr_hashed_link);
      }
      Err(err) => {
        eprintln!("Error processing task: {}", err);
      }
    }
  }
  println!("======================================================================");

  Ok(())
}

pub fn list_link_shortened(link: &String) -> Result<(), Error> {
  let conn = create_connections()?;
  let pattern = format!("%{}%", link);
  let mut stmt = conn.prepare(
    "SELECT link, hashed_link FROM links WHERE hashed_link LIKE ?1"
  )?;
  let link_iter = stmt.query_map(params![pattern], |row| {
    Ok(LinkFormat {
      id: row.get(0)?,
      original_link: row.get(1)?,
      hashed_link: row.get(2)?,
    })
  })?;

  println!("\n                          STORED LINKS\n");
  println!("======================================================================");
  println!("{:<4} | {:<40} | {:<20}", "ID", "Original Link", "Shortened Link");
  println!("----------------------------------------------------------------------");
  for i in link_iter {
    match i {
      Ok(link) => {
        let curr_id = link.id;
        let curr_original_link = crate::operations::display::format_original_link(&link.original_link);

        // Format hashed link
        let curr_hashed_link = crate::operations::display::format_link(&link.hashed_link);

        println!("{:<4} | {:<40} | {}", curr_id, curr_original_link, curr_hashed_link);
      }
      Err(err) => {
        eprintln!("Error processing task: {}", err);
      }
    }
  }
  println!("======================================================================");

  Ok(())
}