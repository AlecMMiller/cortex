use rusqlite::{params, Transaction};

use crate::{database::Delete, models::attribute::GenericAttributeId};

fn check_exists(tx: &Transaction, table: &str, id: &GenericAttributeId) -> rusqlite::Result<bool> {
    let mut stmt = tx.prepare(&format!("SELECT id FROM {table} WHERE id = ?"))?;
    stmt.exists(params![id])
}

fn delete(tx: &Transaction, table: &str, id: &GenericAttributeId) -> rusqlite::Result<()> {
    let mut stmt = tx.prepare(&format!("DELETE FROM {table} WHERE id = ?"))?;
    stmt.execute(params![id])?;
    return Ok(());
}

impl Delete for GenericAttributeId {
    fn delete(self, tx: &Transaction) -> rusqlite::Result<()> {
        let is_text = check_exists(tx, "text_attribute", &self)?;

        if is_text {
            delete(tx, "text_attribute", &self)?;
            return Ok(());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::{params, Error};

    use crate::{
        database::{
            entity::add_entity,
            test::test_util::{setup, ASD, ESD},
            Delete,
        },
        models::{attribute::GenericAttributeId, attribute_schema::Quantity},
    };

    use super::check_exists;

    #[test]
    fn delete_text() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let es = ESD::create_default(&tx);
        let attr = ASD::default().quantity(Quantity::Optional).create(&tx, &es);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr}": "Test"
            }}
            "#
        ))
        .unwrap();

        let parent_id = add_entity(&tx, &es, data).unwrap();

        let id: GenericAttributeId = tx
            .query_row(
                "SELECT id FROM text_attribute WHERE entity = ?",
                params![parent_id],
                |r| r.get(0),
            )
            .unwrap();

        id.clone().delete(&tx).unwrap();

        let exists = check_exists(&tx, "text_attribute", &id).unwrap();
        assert_eq!(exists, false);
    }

    #[test]
    fn delete_required_error() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let es = ESD::create_default(&tx);
        let attr = ASD::default().create(&tx, &es);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr}": "Test"
            }}
            "#
        ))
        .unwrap();

        let parent_id = add_entity(&tx, &es, data).unwrap();

        let id: GenericAttributeId = tx
            .query_row(
                "SELECT id FROM text_attribute WHERE entity = ?",
                params![parent_id],
                |r| r.get(0),
            )
            .unwrap();

        let result = id.clone().delete(&tx);

        assert_eq!(
            result,
            Err(Error::SqliteFailure(
                libsqlite3_sys::Error {
                    code: libsqlite3_sys::ErrorCode::ConstraintViolation,
                    extended_code: 1811
                },
                Some("Cannot delete required field".to_string())
            ))
        );

        let exists = check_exists(&tx, "text_attribute", &id).unwrap();
        assert_eq!(exists, true);
    }
}
