use sqlx::{Error, SqlitePool};

use super::entity::NoteId;

pub async fn create(pool: &SqlitePool, title: &str) -> Result<NoteId, Error> {
    let note_id = NoteId::new();
    let uuid = note_id.to_string();
    let created_at: i64 = super::entity::Timestamp::now().into();
    let body = "".to_string();

    sqlx::query!(
        r#"
        INSERT INTO notes (uuid, title, body, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
        uuid,
        title,
        body,
        created_at,
        created_at
    ).execute(pool).await?;

    Ok(note_id)
}
