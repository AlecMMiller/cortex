use sqlx::{Error, SqlitePool};

use super::entity::{Note, NoteId};

pub async fn get_by_uuid(pool: &SqlitePool, uuid: NoteId) -> Result<Note, Error> {
    let uuid = uuid.to_string();
    let note = sqlx::query_as!(
        Note,
        r#"
        SELECT * FROM notes WHERE uuid = ?
        "#,
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(note)

}

pub async fn get_last_updated(pool: &SqlitePool) -> Option<Note> {
    let note = sqlx::query_as!(
        Note,
        r#"
        SELECT * FROM notes ORDER BY updated_at DESC LIMIT 1
        "#,
    )
    .fetch_one(pool)
    .await;

    match note {
        Ok(note) => Some(note),
        Err(_) => None,
    }
}
