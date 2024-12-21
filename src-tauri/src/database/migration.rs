use rusqlite::{Connection, Result};

fn initial(conn: Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entity_schema (
                id BLOB PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
              )",
        (),
    )?;

    Ok(())
}

fn migrate(conn: Connection) -> Result<()> {
    initial(conn)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate() {
        let conn = Connection::open_in_memory().expect("Could not create db");

        let _result = migrate(conn).expect("Failed to perform migration");
    }
}
