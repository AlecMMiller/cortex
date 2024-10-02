use crate::schema;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = schema::settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Setting {
    pub key: String,
    pub value: String,
}

impl Setting {
    pub fn new(key: &str, value: &str) -> Self {
        Setting {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn as_select() -> (schema::settings::key, schema::settings::value) {
        (schema::settings::key, schema::settings::value)
    }
}
