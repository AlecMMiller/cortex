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

pub struct FullSchema {
    pub uuid: SchemaId,
    pub name: String,
}

create_id!(SchemaId);

#[derive(Type, Serialize)]
pub struct SchemaWithProperties {
    pub uuid: SchemaId,
    pub name: String,
    pub properties: Vec<SchemaProperty>,
}

impl SchemaWithProperties {
    pub fn get(conn: &mut PooledConnection, uuid: &SchemaId) -> Result<(), diesel::result::Error> {
        let _base = SchemaDefinition::get(conn, uuid)?;
        //let properties = SchemaProperty::belonging_to(&base);
        Ok(())
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
}

#[derive(Queryable, Selectable, Insertable, Serialize, Identifiable, Type)]
#[diesel(table_name = crate::schema::schemas)]
#[diesel(primary_key(uuid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SchemaDefinition {
    pub uuid: SchemaId,
    pub name: String,
}

impl SchemaDefinition {
    pub fn get_all(conn: &mut PooledConnection) -> Result<Vec<Self>, diesel::result::Error> {
        schemas::table.select(Self::as_select()).get_results(conn)
    }

    pub fn get(
        conn: &mut PooledConnection,
        uuid: &SchemaId,
    ) -> Result<Self, diesel::result::Error> {
        schemas::table
            .find(uuid)
            .select(Self::as_select())
            .get_result(conn)
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

create_id!(SchemaTextPropertyId);

#[derive(Queryable, Selectable, Serialize, Identifiable, Type, Associations)]
#[diesel(table_name = crate::schema::schema_properties)]
#[diesel(primary_key(uuid))]
#[diesel(belongs_to(crate::schema::schema_definition))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SchemaProperty {
    pub uuid: SchemaTextPropertyId,
    pub name: String,
}
