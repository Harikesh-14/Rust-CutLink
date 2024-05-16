use rusqlite::{Connection, Result};

pub fn create_connections() -> Result<Connection> {
  let conn = Connection::open("storage")?;
  conn.execute(
    "CREATE TABLE IF NOT EXISTS links (
      id INTEGER PRIMARY KEY,
      link TEXT NOT NULL,
      hashed_link TEXT NOT NULL
    )",
    (),
  )?;

  Ok(conn)
}