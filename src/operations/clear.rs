use rusqlite::Error;
use crate::db::conn::create_connections;

pub fn clear_table() -> Result<(), Error> {
  let conn = create_connections()?;

  let result = conn.execute(
    "DELETE FROM links", [],
  );

  match result {
    Ok(_) => {
      println!("All links 🔗 deleted successfully ✅");
      Ok(())
    }
    Err(err) => {
      eprintln!("Error deleting links ❌: {}", err);
      Err(err)
    }
  }
}