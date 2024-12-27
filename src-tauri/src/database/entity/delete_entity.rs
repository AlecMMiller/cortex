use rusqlite::{params, Error, Transaction};

use crate::{database::Delete, models::entity::EntityId};

impl Delete for EntityId {
    fn delete(self, tx: &Transaction) -> rusqlite::Result<()> {
        let mut stmt = tx.prepare(&format!("DELETE FROM entity WHERE id = ?"))?;
        let result = stmt.execute(params![self])?;

        if result == 0 {
            Err(Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::params;

    use crate::database::{
        entity::add_entity,
        test::test_util::{setup, ASD, ESD},
        Delete,
    };

    #[test]
    fn basic_delete() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let es = ESD::create_default(&tx);

        let data = serde_json::from_str(&format!(
            r#"
            {{
            }}
            "#
        ))
        .unwrap();

        let entity_id = add_entity(&tx, &es, data).unwrap();

        entity_id.clone().delete(&tx).unwrap();

        let mut stmt = tx
            .prepare(&format!("SELECT id FROM entity WHERE id = ?"))
            .unwrap();
        let exists = stmt.exists(params![entity_id]).unwrap();

        assert!(!exists);
    }

    #[test]
    fn delete_entity_attrs() {
        let mut conn = setup();

        let tx = conn.transaction().unwrap();

        let es = ESD::create_default(&tx);
        let attr = ASD::create_default(&tx, &es);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr}": "test"
            }}
            "#
        ))
        .unwrap();

        let entity_id = add_entity(&tx, &es, data).unwrap();

        tx.commit().unwrap();
        let tx = conn.transaction().unwrap();

        entity_id.clone().delete(&tx).unwrap();

        let mut stmt = tx
            .prepare(&format!("SELECT id FROM text_attribute WHERE entity = ?"))
            .unwrap();
        let exists = stmt.exists(params![entity_id]).unwrap();

        assert!(!exists);
    }
}
