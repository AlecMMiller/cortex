use chrono::DateTime;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, Output, ToSql},
    sql_types::BigInt,
    sqlite::Sqlite,
};
use serde::Serialize;
use specta::Type;

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Eq, Type)]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub struct AbsoluteTimestamp(pub i64);

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
            }
            None => Err(serde::ser::Error::custom("Could not serialize date")),
        }
    }
}

impl AbsoluteTimestamp {
    pub fn now() -> Self {
        AbsoluteTimestamp(chrono::Utc::now().timestamp())
    }
}
