use rusqlite::{params, OptionalExtension, Transaction};

use crate::{models::longform::TextBlockId, utils::get_timestamp};

fn get_next(tx: &Transaction, id: &TextBlockId) -> rusqlite::Result<Option<TextBlockId>> {
    tx.query_row(
        "SELECT next FROM textblock WHERE id = ?",
        params![id],
        |r| r.get(0),
    )
}

fn get_previous(tx: &Transaction, id: &TextBlockId) -> rusqlite::Result<Option<TextBlockId>> {
    tx.query_row(
        "SELECT id FROM textblock WHERE next = ?",
        params![id],
        |r| r.get(0),
    )
    .optional()
}

fn set_next(tx: &Transaction, block: &TextBlockId, next: &TextBlockId) -> rusqlite::Result<()> {
    tx.execute(
        "UPDATE textblock SET next = ?1 WHERE id = ?2",
        params![next, block],
    )?;

    Ok(())
}

fn create_block(tx: &Transaction) -> rusqlite::Result<(TextBlockId, u64)> {
    let new_id = TextBlockId::new();
    let created_at = get_timestamp();

    tx.execute(
        "INSERT INTO textblock (id, content, created, updated) VALUES (?1, ?2, ?3, ?3)",
        params![new_id, "", created_at],
    )?;

    Ok((new_id, created_at))
}

pub fn create_block_after(tx: &Transaction, block: &TextBlockId) -> rusqlite::Result<TextBlockId> {
    let (middle_id, _created_at) = create_block(tx)?;

    let last_id = get_next(&tx, block)?;
    set_next(tx, block, &middle_id)?;

    match last_id {
        Some(id) => set_next(tx, &middle_id, &id),
        None => Ok(()),
    }?;

    Ok(middle_id)
}

pub fn create_block_before(
    tx: &Transaction,
    after_block: &TextBlockId,
) -> rusqlite::Result<TextBlockId> {
    let (middle_block, _created_at) = create_block(tx)?;

    let maybe_first_block = get_previous(&tx, after_block)?;

    match maybe_first_block {
        Some(first_block) => set_next(tx, &first_block, &middle_block),
        None => {
            tx.execute(
                "UPDATE longform_attribute SET value = ?1 WHERE value = ?2",
                params![&middle_block, after_block],
            )?;
            Ok(())
        }
    }?;

    set_next(tx, &middle_block, after_block)?;

    Ok(middle_block)
}

#[cfg(test)]
mod tests {
    use rusqlite::{params, Transaction};

    use crate::{
        database::{
            attribute::longform::get_next,
            entity::add_entity,
            test::test_util::{setup, ASD, ESD},
        },
        models::{attribute_type::SimpleAttributeType, longform::TextBlockId},
    };

    use super::{create_block_after, create_block_before};

    fn assert_next(tx: &Transaction, id: &TextBlockId, next: &TextBlockId) {
        assert_eq!(get_next(&tx, &id).unwrap(), Some(next.clone()));
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
            "SELECT id FROM textblock WHERE content = 'Hello world'",
            (),
            |r| r.get(0),
        )
        .unwrap()
    }

    #[test]
    fn append_block_at_end() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let first = create_first(&tx);

        let new_id = create_block_after(&tx, &first).unwrap();

        assert_next(&tx, &first, &new_id);
    }

    #[test]
    fn append_block_in_middle() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let first = create_first(&tx);

        let last = create_block_after(&tx, &first).unwrap();

        let middle = create_block_after(&tx, &first).unwrap();

        assert_next(&tx, &first, &middle);
        assert_next(&tx, &middle, &last);
    }

    #[test]
    fn prepend_block_in_middle() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let first = create_first(&tx);

        let last = create_block_after(&tx, &first).unwrap();

        let middle = create_block_before(&tx, &last).unwrap();

        assert_next(&tx, &first, &middle);
        assert_next(&tx, &middle, &last);
    }

    #[test]
    fn prepend_block_at_start() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let existing = create_first(&tx);

        let new_id = create_block_before(&tx, &existing).unwrap();

        assert_next(&tx, &new_id, &existing);

        let mut stmt = tx
            .prepare(&format!(
                "SELECT id FROM longform_attribute WHERE value = ?"
            ))
            .unwrap();
        let exists = stmt.exists(params![new_id]).unwrap();

        assert!(exists);
    }
}
