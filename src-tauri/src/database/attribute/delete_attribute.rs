use rusqlite::{params, Error, Transaction};

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
            return delete(tx, "text_attribute", &self);
        }

        let is_ref = check_exists(tx, "reference_attribute", &self)?;

        if is_ref {
            return delete(tx, "reference_attribute", &self);
        }

        Err(Error::QueryReturnedNoRows)
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::{params, Error};

    use crate::{
        database::{
            entity::{self, add_entity},
            test::test_util::{setup, ASD, ESD, RSD},
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

        let entity_id = add_entity(&tx, &es, data).unwrap();

        let id: GenericAttributeId = tx
            .query_row(
                "SELECT id FROM text_attribute WHERE entity = ?",
                params![entity_id],
                |r| r.get(0),
            )
            .unwrap();

        id.clone().delete(&tx).unwrap();

        let exists = check_exists(&tx, "text_attribute", &id).unwrap();
        assert_eq!(exists, false);
    }

    #[test]
    fn delete_ref() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let es = ESD::create_default(&tx);
        let attr = RSD::default()
            .quantity(Quantity::Optional)
            .create(&tx, &es, &es);

        let data = serde_json::from_str(&format!(
            r#"
            {{
            }}
            "#
        ))
        .unwrap();

        let child = add_entity(&tx, &es, data).unwrap();

        let data = serde_json::from_str(&format!(
            r#"
            {{
            "{attr}": "{child}"
            }}
            "#
        ))
        .unwrap();

        let entity = add_entity(&tx, &es, data).unwrap();

        let id: GenericAttributeId = tx
            .query_row(
                "SELECT id FROM reference_attribute WHERE entity = ?",
                params![entity],
                |r| r.get(0),
            )
            .unwrap();

        id.clone().delete(&tx).unwrap();

        let exists = check_exists(&tx, "reference_attribute", &id).unwrap();
        assert_eq!(exists, false);
    }

    #[test]
    fn delete_not_found_error() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let fake_id = GenericAttributeId::new();

        let result = fake_id.delete(&tx);

        assert_eq!(result, Err(Error::QueryReturnedNoRows));
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
