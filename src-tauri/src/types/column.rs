use sqlx::{Encode, Sqlite, Type};
use uuid::Uuid;

#[allow(dead_code)]
pub enum RawDataType {
    Text,
    Integer,
    Real,
    Blob,
}
pub enum DataType {
    Text,
    RichText,
}
pub struct Column<'a> {
    id: ColumnId,
    pub name: &'a str,
    data_type: DataType,
}

#[derive(Encode)]
pub struct ColumnId (String);

impl<'a> Column<'a> {
    pub fn new(name: &'a str, data_type: DataType) -> Self {
        Self {
            id: ColumnId::new(),
            name,
            data_type,
        }
    }

    pub fn id(&self) -> &ColumnId {
        &self.id
    }

    pub fn get_raw_data_type(&self) -> RawDataType {
        match self.data_type {
            DataType::Text => RawDataType::Text,
            DataType::RichText => RawDataType::Text,
        }
    }
}

impl ColumnId {
    pub fn new() -> Self {
        let id = Uuid::new_v4().simple().to_string();
        let id = format!("u_{}", id);
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }         
}

impl Type<Sqlite> for ColumnId {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}