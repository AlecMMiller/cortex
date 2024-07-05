use uuid::Uuid;

pub struct Schema {
    id: SchemaId,
    name: String
}

pub struct SchemaId (String);

impl SchemaId {
    pub fn new() -> Self {
        let id = Uuid::new_v4().simple().to_string();
        let id = format!("u_{}", id);
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }         
}
