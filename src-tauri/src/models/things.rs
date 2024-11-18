use diesel::prelude::*;
use diesel::serialize::Output;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    serialize::{self, ToSql},
    sql_types::Binary,
    sqlite::Sqlite,
    AsExpression,
};
use serde::{Deserialize, Deserializer, Serialize};
use specta::Type;

use crate::macros::macros::create_id;

create_id!(SchemaId);

#[derive(Queryable, Selectable, Insertable, Serialize, Identifiable, Type)]
#[diesel(table_name = crate::schema::schemas)]
#[diesel(primary_key(uuid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Schema {
    pub uuid: SchemaId,
    pub name: String,
}
