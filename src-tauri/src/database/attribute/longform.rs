use rusqlite::{params, OptionalExtension, Transaction};

use crate::{database::SetValue, models::longform::TextBlockId, utils::get_timestamp};

impl TextBlockId {
    pub fn create_block_before(&self, tx: &Transaction) -> rusqlite::Result<TextBlockId> {
        let (middle_block, _created_at) = create_block(tx)?;

        let maybe_first_block = self.get_previous(&tx)?;

        match maybe_first_block {
            Some(first_block) => first_block.set_next(tx, &middle_block),
            None => {
                tx.execute(
                    "UPDATE longform_attribute SET value = ?1 WHERE value = ?2",
                    params![&middle_block, self],
                )?;
                Ok(())
            }
        }?;

        middle_block.set_next(tx, self)?;

        Ok(middle_block)
    }

    pub fn create_block_after(&self, tx: &Transaction) -> rusqlite::Result<Self> {
        let (middle_id, _created_at) = create_block(tx)?;

        let last_id = self.get_next(&tx)?;
        self.set_next(tx, &middle_id)?;

        match last_id {
            Some(id) => middle_id.set_next(tx, &id),
            None => Ok(()),
        }?;

        Ok(middle_id)
    }

    fn get_next(&self, tx: &Transaction) -> rusqlite::Result<Option<Self>> {
        tx.query_row(
            "SELECT next FROM textblock WHERE id = ?",
            params![self],
            |r| r.get(0),
        )
    }

    fn set_next(&self, tx: &Transaction, next: &TextBlockId) -> rusqlite::Result<()> {
        tx.execute(
            "UPDATE textblock SET next = ?1 WHERE id = ?2",
            params![next, self],
        )?;

        Ok(())
    }

    fn get_previous(&self, tx: &Transaction) -> rusqlite::Result<Option<Self>> {
        tx.query_row(
            "SELECT id FROM textblock WHERE next = ?",
            params![self],
            |r| r.get(0),
        )
        .optional()
    }
}

impl SetValue<&str> for TextBlockId {
    fn set(&self, tx: &Transaction, value: &str) -> rusqlite::Result<()> {
        let updated = get_timestamp();

        tx.execute(
            "UPDATE textblock SET value = ?1, updated = ?2 WHERE id = ?3",
            params![value, updated, self],
        )?;
        Ok(())
    }
}

fn create_block(tx: &Transaction) -> rusqlite::Result<(TextBlockId, u64)> {
    let new_id = TextBlockId::new();
    let created_at = get_timestamp();

    tx.execute(
        "INSERT INTO textblock (id, value, created, updated) VALUES (?1, ?2, ?3, ?3)",
        params![new_id, "", created_at],
    )?;

    Ok((new_id, created_at))
}

#[cfg(test)]
mod tests {
    use rusqlite::{params, Transaction};

    use crate::{
        database::{
            entity::add_entity,
            test::test_util::{setup, ASD, ESD},
            SetValue,
        },
        models::{attribute_type::SimpleAttributeType, longform::TextBlockId},
    };

    #[test]
    fn set_content() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let block = create_first(&tx);

        block.set(&tx, "test").unwrap();

        let val: String = tx
            .query_row(
                "SELECT value FROM textblock WHERE id = ?",
                params![block],
                |r| r.get(0),
            )
            .unwrap();

        assert_eq!(val, "test");
    }

    #[test]
    fn append_block_at_end() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let first = create_first(&tx);

        let new_id = first.create_block_after(&tx).unwrap();

        assert_next(&tx, &first, &new_id);
    }

    #[test]
    fn append_block_in_middle() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let first = create_first(&tx);

        let last = first.create_block_after(&tx).unwrap();

        let middle = first.create_block_after(&tx).unwrap();

        assert_next(&tx, &first, &middle);
        assert_next(&tx, &middle, &last);
    }

    #[test]
    fn prepend_block_in_middle() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let first = create_first(&tx);

        let last = first.create_block_after(&tx).unwrap();

        let middle = last.create_block_before(&tx).unwrap();

        assert_next(&tx, &first, &middle);
        assert_next(&tx, &middle, &last);
    }

    #[test]
    fn prepend_block_at_start() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let existing = create_first(&tx);

        let new_id = existing.create_block_before(&tx).unwrap();

        assert_next(&tx, &new_id, &existing);

        let mut stmt = tx
            .prepare(&format!(
                "SELECT id FROM longform_attribute WHERE value = ?"
            ))
            .unwrap();
        let exists = stmt.exists(params![new_id]).unwrap();

        assert!(exists);
    }

    fn assert_next(tx: &Transaction, id: &TextBlockId, next: &TextBlockId) {
        assert_eq!(id.get_next(&tx).unwrap(), Some(next.clone()));
    }

    fn create_first(tx: &Transaction) -> TextBlockId {
        let schema = ESD::create_default(&tx);
        let attr = ASD::default()
            .attr_type(SimpleAttributeType::Longform)
            .create(&tx, &schema);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr}": "Hello world"
            }}
            "#
        ))
        .unwrap();

        add_entity(&tx, &schema, data).unwrap();
        tx.query_row(
            "SELECT id FROM textblock WHERE value = 'Hello world'",
            (),
            |r| r.get(0),
        )
        .unwrap()
    }
}
