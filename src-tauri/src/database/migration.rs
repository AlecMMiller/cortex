use rusqlite::{Result, Transaction};

fn create_table(tx: &Transaction, name: &str, content: &str) -> Result<()> {
    let content = format!(
        "
        CREATE TABLE IF NOT EXISTS {name} (
          id BLOB PRIMARY KEY,
          {content}
        );
        "
    );
    tx.execute(&content, ())?;

    Ok(())
}

fn build_attr(tx: &Transaction, name: &str, sql_type: &str) -> Result<()> {
    create_table(
        tx,
        &format!("{name}_attribute"),
        &format!(
            "
            entity BLOB NOT NULL,
            schema BLOB NOT NULL,
            value {sql_type} NOT NULL,
            FOREIGN KEY(entity) REFERENCES entity(id),
            FOREIGN KEY(schema) REFERENCES attribute_schema(id)
            "
        ),
    )?;

    tx.execute(
        &format!("CREATE INDEX IF NOT EXISTS idx_{name}_entity_schema ON {name}_attribute (entity, schema);"),
        (),
    )?;
    Ok(())
}

fn initial(conn: &Transaction) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entity_schema (
                id BLOB PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
              )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS attribute_schema (
                id BLOB PRIMARY KEY,
                entity BLOB NOT NULL,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                reference BLOB,
                quantity TEXT NOT NULL,
                UNIQUE(entity, name),
                FOREIGN KEY(reference) REFERENCES entity_schema(id),
                FOREIGN KEY(entity) REFERENCES entity_schema(id)
              );",
        (),
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_entity_attributes ON attribute_schema (entity);",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entity (
                id BLOB PRIMARY KEY,
                schema BLOB NOT NULL,
                FOREIGN KEY(schema) REFERENCES entity_schema(id)
              )",
        (),
    )?;

    build_attr(&conn, "text", "TEXT")?;
    build_attr(&conn, "integer", "INTEGER")?;
    build_attr(&conn, "number", "REAL")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reference_attribute (
              id BLOB PRIMARY KEY,
              entity BLOB NOT NULL,
              schema BLOB NOT NULL,
              value TEXT NOT NULL,
              FOREIGN KEY(entity) REFERENCES entity(id),
              FOREIGN KEY(schema) REFERENCES attribute_schema(id),
              FOREIGN KEY(value) REFERENCES entity(id)
            );",
        (),
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_ref_entity_schema ON reference_attribute (entity, schema);",
        (),
    )?;

    Ok(())
}

#[allow(dead_code)]
pub fn migrate(conn: &Transaction) -> Result<()> {
    initial(&conn)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn test_migrate() {
        let mut conn = Connection::open_in_memory().expect("Could not create db");
        let tx = conn.transaction().unwrap();

        migrate(&tx).expect("Failed to perform migration");
    }

    // Test that running a migration again won't cause issues
    #[test]
    fn test_double_migrate() {
        let mut conn = Connection::open_in_memory().expect("Could not create db");
        let tx = conn.transaction().unwrap();

        migrate(&tx).expect("Failed to perform migration");
        tx.commit().unwrap();

        let tx = conn.transaction().unwrap();
        migrate(&tx).unwrap();
    }
}
