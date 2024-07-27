use chrono::DateTime;
use diesel::{backend::Backend, deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, prelude::{Insertable, Queryable}, serialize::{self, Output, ToSql}, sql_types::{BigInt, Binary}, sqlite::Sqlite, Selectable};
use serde::Serialize;
use crate::macros::macros::create_id;

create_id!(NoteId);

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Eq)]
#[sql_type = "diesel::sql_types::BigInt"]
pub struct AbsoluteTimestamp(i64);

impl FromSql<BigInt, Sqlite> for AbsoluteTimestamp {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(AbsoluteTimestamp {
            0: i64::from_sql(bytes)?,
        })
    }
}

impl ToSql<BigInt, Sqlite> for AbsoluteTimestamp {
    fn to_sql<'a>(&'a self, out: &mut Output<'a, '_, Sqlite>) -> serialize::Result {
        ToSql::<BigInt, Sqlite>::to_sql(&self.0, out)
    }
}

impl Serialize for AbsoluteTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let date = DateTime::from_timestamp(self.0, 0);

        match date {
            Some(date) => {
                let s = date.to_rfc3339();
                serializer.serialize_str(&s)
            },
            None => {
                Err(serde::ser::Error::custom("Could not serialize date"))
            }
        }
    }
}

impl AbsoluteTimestamp {
    pub fn now() -> Self {
        AbsoluteTimestamp(chrono::Utc::now().timestamp())
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    pub uuid: NoteId,
    pub title: String,
    pub body: String,
    pub created_at: AbsoluteTimestamp,
    pub updated_at: AbsoluteTimestamp,
}

impl Note {
    pub fn new(title: &str, body: &str) -> Self {
        let time = chrono::Utc::now().timestamp();
        Note {
            uuid: NoteId::new(),
            title: title.to_string(),
            body: body.to_string(),
            created_at: AbsoluteTimestamp(time),
            updated_at: AbsoluteTimestamp(time),
        }
    }

    pub fn as_select() -> (crate::schema::notes::uuid, crate::schema::notes::title, crate::schema::notes::body, crate::schema::notes::created_at, crate::schema::notes::updated_at) {
        (crate::schema::notes::uuid, crate::schema::notes::title, crate::schema::notes::body, crate::schema::notes::created_at, crate::schema::notes::updated_at)
    }
}
