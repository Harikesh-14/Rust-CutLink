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

  for i in links_iter {
    match i {
      Ok(link) => {
        let curr_id = link.id;
        let curr_original_link = link.original_link;
        let curr_hashed_link = link.hashed_link;

        println!("[{}]. {} --> cut.link/{}", curr_id, curr_original_link, curr_hashed_link);
      }

      Err(err) => {
        eprintln!("Error processing task: {}", err);
      }
    }
  }

  Ok(())
}
