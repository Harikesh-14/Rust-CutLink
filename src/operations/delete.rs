use rusqlite::Error;
use crate::db::conn::create_connections;

pub fn delete_link(code: i32) -> Result<(), Error> {
  let mut conn = create_connections()?;

  let result = conn.execute("DELETE FROM links WHERE id = ?1", &[&code]);

  match result {
    Ok(deleted_count) => {
      if deleted_count > 0 {
        let tx = conn.transaction()?;
        tx.execute("UPDATE links SET id = id - 1 WHERE id > ?1", &[&code])?;
        tx.commit()?;

        println!("\n=====================================================================");
        println!("                    Link Deletion Successful");
        println!("-----------------------------------------------------------------------");
        println!("  Link with ID {} has been deleted successfully.", code);
        println!("=======================================================================\n");
      } else {
        println!("\n=====================================================================");
        println!("                    Link Deletion Unsuccessful");
        println!("-----------------------------------------------------------------------");
        println!("  No link found with ID {}.", code);
        println!("=====================================================================\n");
      }
      Ok(())
    }
    Err(err) => {
      eprintln!("Error deleting the link: {}", err);
      Err(err)
    }
  }
}
