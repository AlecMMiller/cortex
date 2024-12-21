use crate::macros::macros::create_id;
use rusqlite::{params, types::FromSql, Connection, Result, ToSql};
use serde::{Deserialize, Deserializer, Serialize};

create_id!(EntitySchemaId);

struct EntitySchema {
    id: EntitySchemaId,
    name: String,
}

impl EntitySchema {
    pub fn new(conn: &Connection, name: String) -> Result<Self> {
        let new_entity_schema = Self {
            id: EntitySchemaId::new(),
            name,
        };

        conn.execute(
            "INSERT INTO entity_schema (id, name) VALUES (?1, ?2)",
            (&new_entity_schema.id, &new_entity_schema.name),
        )?;

        Ok(new_entity_schema)
    }

    pub fn get(conn: &Connection, id: &EntitySchemaId) -> Result<Self> {
        conn.query_row(
            "SELECT id, name FROM entity_schema WHERE id=?1",
            params![id],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::database::test::test_util::setup;

    use super::*;

    #[test]
    fn test_new() {
        let conn = setup();
        let name = "Foo";

        let new = EntitySchema::new(&conn, name.to_string()).expect("Unable to create entity");
        let id = new.id;

        let stored = EntitySchema::get(&conn, &id).expect("Could not get stored");

        assert_eq!(stored.id, id);
        assert_eq!(stored.name, name);
    }
}
