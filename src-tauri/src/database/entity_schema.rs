use crate::macros::macros::create_id;
use serde::{Deserialize, Deserializer, Serialize};

create_id!(EntitySchemaId);

struct EntitySchema<'a> {
    id: EntitySchemaId,
    name: &'a str,
}

impl<'a> EntitySchema<'a> {
    pub fn new(name: &'a str) -> Self {
        let new_entity_schema = Self {
            id: EntitySchemaId::new(),
            name,
        };

        new_entity_schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = "Foo";

        let new = EntitySchema::new(name);

        assert_eq!(new.name, name);
    }
}
