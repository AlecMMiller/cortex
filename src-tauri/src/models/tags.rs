use super::notes::Note;
use crate::{macros::macros::create_id, utils::PooledConnection};
use diesel::{
    associations::HasTable,
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    result::Error,
    serialize::{self, Output, ToSql},
    sql_types::Binary,
    sqlite::Sqlite,
    Selectable,
};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::schema::{note_tags, tag_parents, tags};

use super::notes::NoteId;

create_id!(TagId);

#[derive(Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable, Serialize, Type)]
#[diesel(table_name = tags)]
#[diesel(primary_key(uuid))]
pub struct Tag {
    pub uuid: TagId,
    pub title: String,
}

#[derive(Selectable, Queryable, Insertable, Identifiable, Associations, Debug)]
#[diesel(table_name = note_tags)]
#[diesel(belongs_to(Tag, foreign_key = tag_uuid))]
#[diesel(belongs_to(Note, foreign_key = note_uuid))]
#[diesel(primary_key(tag_uuid, note_uuid))]
pub struct NoteTag<'a> {
    pub tag_uuid: &'a TagId,
    pub note_uuid: &'a NoteId,
}

impl<'a> NoteTag<'a> {
    pub fn new(
        conn: &mut PooledConnection,
        note_uuid: &'a NoteId,
        tag_uuid: &'a TagId,
    ) -> Result<Self, Error> {
        let new_link = Self {
            tag_uuid,
            note_uuid,
        };

        diesel::insert_into(note_tags::table)
            .values(&new_link)
            .execute(conn)?;

        Ok(new_link)
    }
}

#[derive(Selectable, Insertable, Queryable, Identifiable, Associations, Debug)]
#[diesel(table_name = tag_parents)]
#[diesel(belongs_to(Tag, foreign_key = parent_uuid))]
#[diesel(primary_key(parent_uuid, child_uuid))]
pub struct TagParent {
    pub parent_uuid: TagId,
    pub child_uuid: TagId,
}

impl Tag {
    pub fn new(conn: &mut PooledConnection, title: &str) -> Result<Self, Error> {
        let new_tag = Self {
            uuid: TagId::new(),
            title: title.to_string(),
        };

        diesel::insert_into(tags::table)
            .values(&new_tag)
            .execute(conn)?;

        Ok(new_tag)
    }

    pub fn get(conn: &mut PooledConnection, uuid: &TagId) -> Result<Self, Error> {
        tags::table
            .find(uuid)
            .select(Tag::as_select())
            .get_result(conn)
    }

    pub fn get_available_containing(
        conn: &mut PooledConnection,
        content: &str,
        max_results: i64,
        note_uuid: NoteId,
    ) -> Result<(Vec<Self>, bool), Error> {
        let already_has = note_tags::table
            .filter(note_tags::note_uuid.eq(note_uuid))
            .select(note_tags::tag_uuid)
            .into_boxed();

        let available = tags::table
            .filter(tags::uuid.ne_all(already_has))
            .filter(tags::title.like(format!("{content}%")))
            .order(tags::title.asc())
            .select(Tag::as_select())
            .limit(max_results)
            .load(conn)?;

        let exists = match tags::table
            .filter(tags::title.like(content))
            .select(tags::uuid)
            .first::<TagId>(conn)
            .optional()?
        {
            Some(..) => true,
            None => false,
        };

        Ok((available, exists))
    }

    pub fn rename(conn: &mut PooledConnection, uuid: &TagId, title: &str) -> Result<(), Error> {
        diesel::update(tags::table.find(uuid))
            .set(tags::title.eq(title))
            .execute(conn)?;

        Ok(())
    }

    pub fn get_direct_by_note(
        conn: &mut PooledConnection,
        uuid: &NoteId,
    ) -> Result<Vec<Self>, Error> {
        Ok(NoteTag::table()
            .filter(note_tags::note_uuid.eq(uuid))
            .inner_join(tags::table)
            .select(Tag::as_select())
            .load(conn)?)
    }
}
