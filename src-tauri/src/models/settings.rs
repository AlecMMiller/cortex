use crate::{
    schema::{self, settings},
    utils::PooledConnection,
};
use diesel::{prelude::*, result::Error, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Serialize, Identifiable)]
#[diesel(table_name = schema::settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(key))]
pub struct Setting {
    pub key: String,
    pub value: String,
}

impl Setting {
    pub fn get(conn: &mut PooledConnection, key: &str) -> Result<Self, Error> {
        settings::table
            .find(key)
            .select(Setting::as_select())
            .get_result(conn)
    }

    pub fn get_or_set(conn: &mut PooledConnection, key: &str, value: &str) -> Result<Self, Error> {
        let existing = settings::table
            .find(key)
            .select(Setting::as_select())
            .first(conn)
            .optional()?;

        match existing {
            Some(setting) => Ok(setting),
            None => {
                let setting = Setting {
                    key: key.to_string(),
                    value: value.to_string(),
                };

                if value == "" {
                    println!("Empty value for setter, ignoring");
                    return Ok(setting);
                }

                println!("No existing value found, setting to {value}");

                diesel::insert_into(settings::table)
                    .values(&setting)
                    .execute(conn)?;

                Ok(setting)
            }
        }
    }

    pub fn set(conn: &mut PooledConnection, key: &str, value: &str) -> Result<(), Error> {
        diesel::update(settings::table.find(key))
            .set(settings::value.eq(value))
            .execute(conn)?;

        Ok(())
    }

    pub fn as_select() -> (schema::settings::key, schema::settings::value) {
        (schema::settings::key, schema::settings::value)
    }
}
