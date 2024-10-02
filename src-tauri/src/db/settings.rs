use crate::schema::settings::dsl::settings;
use crate::schema::settings::table;
use crate::utils::get_connection;
use crate::{models::settings::Setting, SqlitePool};
use diesel::{prelude::*, result::Error};

pub fn get(pool: SqlitePool, setting_key: &str) -> Result<Setting, Error> {
    let conn = &mut get_connection(pool);

    let setting = settings
        .find(setting_key)
        .select(Setting::as_select())
        .first(conn)
        .optional();

    match setting {
        Ok(Some(setting)) => Ok(setting),
        Ok(None) => Err(Error::NotFound),
        Err(foo) => Err(foo),
    }
}

pub fn get_or_set(pool: SqlitePool, setting_key: &str, value: &str) -> Result<Setting, Error> {
    let conn = &mut get_connection(pool);

    println!("Requested setting for {setting_key}");

    let existing = settings
        .find(setting_key)
        .select(Setting::as_select())
        .first(conn)
        .optional()?;

    match existing {
        Some(setting) => Ok(setting),
        None => {
            let setting = Setting {
                key: setting_key.to_string(),
                value: value.to_string(),
            };

            if value == "" {
                println!("Empty value for setter, ignoring");
                return Ok(setting);
            }

            println!("No existing value found, setting to {value}");

            diesel::insert_into(table).values(&setting).execute(conn)?;

            Ok(setting)
        }
    }
}
