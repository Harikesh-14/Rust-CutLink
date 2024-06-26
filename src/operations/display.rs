use rusqlite::Error;
use crate::db::conn::create_connections;

struct LinkFormat {
  id: i32,
  original_link: String,
  hashed_link: String,
}

pub fn display_stored_links() -> Result<(), Error> {
  let conn = create_connections()?;
  let mut stmt = conn.prepare("SELECT * FROM links")?;
  let links_iter = stmt.query_map([], |row| {
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
  for i in links_iter {
    match i {
      Ok(link) => {
        let curr_id = link.id;
        let curr_original_link = format_original_link(&link.original_link);

        // Format hashed link
        let curr_hashed_link = format_link(&link.hashed_link);

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

pub(crate) fn format_original_link(link: &str) -> String {
  if link.len() <= 40 {
    link.to_string()
  } else {
    let truncated_link = &link[..37];
    format!("{}...", truncated_link)
  }
}

pub(crate) fn format_link(link: &str) -> String {
  format!("cut.link/{}", link)
}
