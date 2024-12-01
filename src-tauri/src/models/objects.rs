use crate::macros::macros::create_id;
use crate::schema::schemas;
use crate::utils::PooledConnection;
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

create_id!(SchemaId);

#[derive(Queryable, Selectable, Insertable, Serialize, Identifiable, Type)]
#[diesel(table_name = crate::schema::schemas)]
#[diesel(primary_key(uuid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Schema {
    pub uuid: SchemaId,
    pub name: String,
}

impl Schema {
    pub fn get_all(conn: &mut PooledConnection) -> Result<Vec<Self>, diesel::result::Error> {
        schemas::table.select(Self::as_select()).get_results(conn)
    }

    pub fn get(
        conn: &mut PooledConnection,
        uuid: &SchemaId,
    ) -> Result<Self, diesel::result::Error> {
        schemas::table
            .find(uuid)
            .select(Schema::as_select())
            .get_result(conn)
    }

    pub fn rename(
        conn: &mut PooledConnection,
        uuid: &SchemaId,
        name: &str,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(schemas::table.find(uuid))
            .set(schemas::name.eq(name))
            .execute(conn)?;

        Ok(())
    }

    pub fn new(conn: &mut PooledConnection, name: &str) -> Result<Self, diesel::result::Error> {
        let new_schema = Self {
            uuid: SchemaId::new(),
            name: name.to_string(),
        };

        diesel::insert_into(schemas::table)
            .values(&new_schema)
            .execute(conn)?;

        Ok(new_schema)
    }
}
