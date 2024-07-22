use sqlx::{sqlite::SqliteQueryResult, Error, SqlitePool};

use super::entity::Timestamp;

pub async fn rename_note(pool: &SqlitePool, uuid: &str, title: &str) -> Result<SqliteQueryResult, Error> {
    let updated_at = Timestamp::now();
    let updated_at: i64 = updated_at.into();
    
    sqlx::query!(
        r#"
        UPDATE notes SET title = ?, updated_at = ? WHERE uuid = ?
        "#,
        title,
        updated_at,
        uuid
    ).execute(pool).await
}

pub async fn update_note_body(pool: &SqlitePool, uuid: &str, body: &str) -> Result<SqliteQueryResult, Error> {
    let updated_at = Timestamp::now();
    let updated_at: i64 = updated_at.into();
    
    sqlx::query!(
        r#"
        UPDATE notes SET body = ?, updated_at = ? WHERE uuid = ?
        "#,
        body,
        updated_at,
        uuid
    ).execute(pool).await
}
