use rusqlite::{Result, Transaction};

fn create_table(tx: &Transaction, name: &str, content: &str) -> Result<()> {
    let content = format!(
        "
        CREATE TABLE IF NOT EXISTS {name} (
          id BLOB PRIMARY KEY,
          created INTEGER NOT NULL,
          updated INTEGER NOT NULL,
          {content}
        );
        "
    );
    tx.execute(&content, ())?;

    Ok(())
}

fn build_attr(tx: &Transaction, name: &str, sql_type: &str, extra: &str) -> Result<()> {
    create_table(
        tx,
        &format!("{name}_attribute"),
        &format!(
            "
            schema BLOB NOT NULL,
            value {sql_type} NOT NULL,
            entity BLOB NOT NULL REFERENCES entity (id) ON DELETE CASCADE,
            {extra}
            FOREIGN KEY(schema) REFERENCES attribute_schema(id)
            "
        ),
    )?;

    tx.execute(
        &format!("CREATE INDEX IF NOT EXISTS idx_{name}_entity_schema ON {name}_attribute (entity, schema);"),
        (),
    )?;

    // When a new entry is added, check if this is a List attribute or not
    // If not, prevent more than one entry being added
    tx.execute(
        &format!(
            "
            CREATE TRIGGER IF NOT EXISTS {name}_single_check
            BEFORE INSERT ON {name}_attribute
              WHEN NOT EXISTS ( SELECT 1 FROM attribute_schema WHERE ID = NEW.schema AND quantity = 'List' )
              AND EXISTS ( SELECT 1 FROM {name}_attribute WHERE entity = NEW.entity AND schema = NEW.schema ) 
            BEGIN
              SELECT RAISE(FAIL, \"Attempted to add second entry to non-list field\");
            END;
            "
        ),
        (),
    )?;

    // Checks if the field is required, and if so prevent it being deleted
    // However, we need to check if the parent exists, if it doesn't that
    // indicates it's in the process of being deleted and this is a CASCADE
    // delete, so we allow the deletion regardless
    tx.execute(
        &format!(
            "
            CREATE TRIGGER IF NOT EXISTS {name}_required_check
            BEFORE DELETE ON {name}_attribute
              WHEN EXISTS ( SELECT 1 FROM attribute_schema WHERE id = OLD.schema AND quantity = 'Required' )
              AND EXISTS (SELECT 1 FROM entity WHERE id = OLD.entity ) -- handle case where parent entity is being deleted
            BEGIN
              SELECT RAISE(FAIL, \"Cannot delete required field\");
            END;
            "
        ),
        (),
    )?;

    Ok(())
}

fn initial(tx: &Transaction) -> Result<()> {
    create_table(
        &tx,
        "entity_schema",
        "
        name TEXT NOT NULL UNIQUE        
        ",
    )?;

    create_table(
        tx,
        "attribute_schema",
        "
      entity BLOB NOT NULL,
      name TEXT NOT NULL,
      type TEXT NOT NULL,
      reference BLOB,
      quantity TEXT NOT NULL,
      UNIQUE(entity, name),
      FOREIGN KEY(reference) REFERENCES entity_schema(id) ON DELETE CASCADE,
      FOREIGN KEY(entity) REFERENCES entity_schema(id) ON DELETE CASCADE
      ",
    )?;

    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_entity_attributes ON attribute_schema (entity);",
        (),
    )?;

    create_table(
        tx,
        "entity",
        "
        schema BLOB NOT NULL,
        FOREIGN KEY(schema) REFERENCES entity_schema(id)
      ",
    )?;

    create_table(
        tx,
        "textblock",
        "
        next BLOB UNIQUE,
        value TEXT NOT NULL,
        FOREIGN KEY(next) REFERENCES textblock(id)
      ",
    )?;

    build_attr(&tx, "text", "TEXT", "")?;
    build_attr(&tx, "integer", "INTEGER", "")?;
    build_attr(&tx, "number", "REAL", "")?;

    build_attr(
        &tx,
        "reference",
        "BLOB",
        "FOREIGN KEY(value) REFERENCES entity(id),",
    )?;

    build_attr(
        &tx,
        "longform",
        "BLOB",
        "FOREIGN KEY(value) REFERENCES textblock(id),",
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
