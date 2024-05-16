use rusqlite::Error;
use crate::db::conn::create_connections;

pub fn delete_link(code: i32) -> Result<(), Error> {
  let mut conn = create_connections()?;

  let result = conn.execute("DELETE FROM links WHERE id = ?1", &[&code]);

  match result {
    Ok(_) => {
      let tx = conn.transaction()?;
      tx.execute("UPDATE links SET id = id - 1 WHERE id > ?1", &[&code])?;
      tx.commit()?;

      println!("Link with ID {} deleted successfully", code);
      Ok(())
    }
    Err(err) => {
      eprintln!("Error deleting the link: {}", err);
      Err(err)
    }
  }
}