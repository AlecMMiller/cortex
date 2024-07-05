use uuid::Uuid;

pub struct ColumnId (String);

impl ColumnId {
    pub fn new() -> Self {
        let id = Uuid::new_v4().simple().to_string();
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }         
}