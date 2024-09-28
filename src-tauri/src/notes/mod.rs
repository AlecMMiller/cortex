use crate::{
    models::{AbsoluteTimestamp, Note, NoteId},
    schema::notes::{body, dsl::notes, title, updated_at},
    utils::get_connection,
    SqlitePool,
};
use diesel::{prelude::*, result::Error};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Queryable, Selectable)]
#[diesel(table_name = crate::schema::notes)]
pub struct NoteTitle {
    uuid: NoteId,
    title: String,
}

pub fn get_by_uuid(pool: SqlitePool, uuid: NoteId) -> Result<Note, Error> {
    let conn = &mut get_connection(pool);

    let note = notes
        .find(uuid)
        .select(Note::as_select())
        .first(conn)
        .optional();

    match note {
        Ok(Some(note)) => Ok(note),
        Ok(None) => Err(Error::NotFound),
        Err(foo) => Err(foo),
    }
}

pub fn get_all(pool: SqlitePool) -> Result<Vec<NoteTitle>, Error> {
    let conn = &mut get_connection(pool);

    let all_notes = notes.select(NoteTitle::as_select()).get_results(conn)?;

    Ok(all_notes)
}

pub enum GetLatestError {
    UnknownError,
}

pub fn get_last_updated_or_create(pool: SqlitePool) -> Result<Note, GetLatestError> {
    let conn = &mut get_connection(pool);

    let note: Result<Note, Error> = conn.immediate_transaction(|conn| {
        let note = notes
            .order(updated_at.desc())
            .select(Note::as_select())
            .first(conn)
            .optional()?;

        match note {
            Some(note) => Ok(note),
            None => {
                println!("No notes found, creating new note");
                let new_note = Note::new("New Note", "");
                diesel::insert_into(notes).values(&new_note).execute(conn)?;
                Ok(new_note)
            }
        }
    });

    match note {
        Ok(note) => Ok(note),
        Err(_) => Err(GetLatestError::UnknownError),
    }
}

pub fn rename_note(pool: SqlitePool, uuid: NoteId, new_title: &str) -> Result<(), Error> {
    let time = AbsoluteTimestamp::now();
    let conn = &mut get_connection(pool);

    diesel::update(notes.find(uuid))
        .set((title.eq(new_title), updated_at.eq(time)))
        .execute(conn)?;

    Ok(())
}

pub fn create_note(pool: SqlitePool, new_title: &str) -> Result<Note, Error> {
    let new_note = Note::new(new_title, "");
    let conn = &mut get_connection(pool);

    diesel::insert_into(notes).values(&new_note).execute(conn)?;

    Ok(new_note)
}

pub fn update_body(pool: SqlitePool, uuid: NoteId, new_body: &str) -> Result<(), Error> {
    let time = AbsoluteTimestamp::now();
    let conn = &mut get_connection(pool);

    diesel::update(notes.find(uuid))
        .set((updated_at.eq(time), body.eq(new_body)))
        .execute(conn)?;

    Ok(())
}
