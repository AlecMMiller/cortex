pub use super::time::AbsoluteTimestamp;
use crate::lexical::{EditorState, GetRawText};
use crate::search::{TextIndexWriter, CONTENT, ID, TITLE};
use crate::{macros::macros::create_id, schema::notes};
use crate::{schema, utils::PooledConnection};
use diesel::prelude::Identifiable;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    prelude::{Insertable, Queryable},
    serialize::{self, Output, ToSql},
    sql_types::Binary,
    sqlite::Sqlite,
    Selectable,
};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tantivy::{TantivyDocument, Term};
create_id!(NoteId);

#[derive(Queryable, Selectable, Insertable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(primary_key(uuid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    pub uuid: NoteId,
    pub title: String,
    pub body: String,
    pub created_at: AbsoluteTimestamp,
    pub updated_at: AbsoluteTimestamp,
}

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    Tantivy(tantivy::TantivyError),
    Serde(serde_json::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::Diesel(err)
    }
}

impl From<tantivy::TantivyError> for Error {
    fn from(err: tantivy::TantivyError) -> Self {
        Self::Tantivy(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

impl Note {
    pub fn new(
        conn: &mut PooledConnection,
        writer: Arc<TextIndexWriter>,
        title: &str,
        body: &str,
    ) -> Result<Self, diesel::result::Error> {
        let time = chrono::Utc::now().timestamp();
        let new_note = Self {
            uuid: NoteId::new(),
            title: title.to_string(),
            body: body.to_string(),
            created_at: AbsoluteTimestamp(time),
            updated_at: AbsoluteTimestamp(time),
        };

        diesel::insert_into(notes::table)
            .values(&new_note)
            .execute(conn)?;

        let _ = Self::on_note_change(conn, writer, &new_note.uuid);

        Ok(new_note)
    }

    pub fn get(conn: &mut PooledConnection, uuid: &NoteId) -> Result<Self, diesel::result::Error> {
        notes::table
            .find(uuid)
            .select(Note::as_select())
            .get_result(conn)
    }

    pub fn get_all(conn: &mut PooledConnection) -> Result<Vec<Self>, diesel::result::Error> {
        notes::table.select(Self::as_select()).get_results(conn)
    }

    pub fn rename(
        conn: &mut PooledConnection,
        index: Arc<TextIndexWriter>,
        uuid: &NoteId,
        new_title: &str,
    ) -> Result<(), Error> {
        let time = AbsoluteTimestamp::now();

        diesel::update(notes::table.find(&uuid))
            .set((notes::title.eq(new_title), notes::updated_at.eq(time)))
            .execute(conn)?;

        let _ = Self::on_note_change(conn, index, uuid);

        Ok(())
    }

    pub fn update_body(
        conn: &mut PooledConnection,
        index: Arc<TextIndexWriter>,
        uuid: &NoteId,
        new_body: &str,
    ) -> Result<(), diesel::result::Error> {
        let time = AbsoluteTimestamp::now();

        diesel::update(notes::table.find(&uuid))
            .set((notes::updated_at.eq(time), notes::body.eq(new_body)))
            .execute(conn)?;

        let _ = Self::on_note_change(conn, index, uuid);

        Ok(())
    }

    fn on_note_change(
        conn: &mut PooledConnection,
        writer: Arc<TextIndexWriter>,
        uuid: &NoteId,
    ) -> Result<(), Error> {
        let note = Self::get(conn, uuid)?;

        let schema = writer.schema.clone();
        let reader = writer.reader.clone();
        let mut writer = writer.writer.lock().unwrap();

        let title = schema.get_field(TITLE)?;
        let id = schema.get_field(ID)?;
        let content = schema.get_field(CONTENT)?;

        let uuid = &note.uuid.to_string();

        let uuid_term = Term::from_field_text(id, uuid);
        writer.delete_term(uuid_term.clone());

        let mut doc = TantivyDocument::default();

        let body = note.body;
        let body: EditorState = serde_json::from_str(&body)?;
        let body = match body.get_raw_text() {
            Some(body) => body,
            None => "".to_string(),
        };

        doc.add_text(title, note.title);
        doc.add_text(id, uuid);
        doc.add_text(content, body);

        writer.add_document(doc)?;

        writer.commit()?;
        reader.reload()?;

        Ok(())
    }

    pub fn as_select() -> (
        schema::notes::uuid,
        schema::notes::title,
        schema::notes::body,
        schema::notes::created_at,
        schema::notes::updated_at,
    ) {
        (
            schema::notes::uuid,
            schema::notes::title,
            schema::notes::body,
            schema::notes::created_at,
            schema::notes::updated_at,
        )
    }
}

#[derive(Serialize, Debug, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(primary_key(uuid))]
pub struct NoteTitle {
    pub uuid: NoteId,
    pub title: String,
}

impl NoteTitle {
    pub fn get_all(conn: &mut PooledConnection) -> Result<Vec<Self>, diesel::result::Error> {
        notes::table.select(Self::as_select()).get_results(conn)
    }
}
