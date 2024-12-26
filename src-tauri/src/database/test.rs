#[cfg(test)]
pub mod test_util {
    use rusqlite::{Connection, Transaction};
    use serde_json::{Map, Value};

    use crate::{
        database::{
            entity_schema::{CreateEntitySchema, EntitySchema, EntitySchemaId},
            migration::migrate,
            New,
        },
        models::{
            attribute_schema::{
                AttributeSchema, AttributeSchemaId, CreateAttributeSchema, Quantity,
            },
            attribute_type::{CreateAttributeType, CreateReferenceAttribute, SimpleAttributeType},
        },
    };

    pub fn setup() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();

        let tx = conn.transaction().unwrap();
        migrate(&tx).unwrap();
        tx.commit().unwrap();

        conn
    }

    pub struct ESD {
        pub name: String,
    }

    impl Default for ESD {
        fn default() -> Self {
            Self {
                name: "Foo".to_string(),
            }
        }
    }

    pub fn create_entity_schema(tx: &Transaction, definition: ESD) -> EntitySchemaId {
        let schema = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: definition.name,
            },
        )
        .expect("Unable to create entity");

        schema.id
    }

    pub struct ASD {
        name: String,
        attr_type: SimpleAttributeType,
        quantity: Quantity,
    }

    impl ASD {
        pub fn name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }

        #[allow(unused)]
        pub fn attr_type(mut self, attr_type: SimpleAttributeType) -> Self {
            self.attr_type = attr_type;
            self
        }

        pub fn quantity(mut self, quantity: Quantity) -> Self {
            self.quantity = quantity;
            self
        }
    }

    impl Default for ASD {
        fn default() -> Self {
            Self {
                name: "Foo".to_string(),
                attr_type: SimpleAttributeType::Text,
                quantity: Quantity::Required,
            }
        }
    }

    pub fn create_attribute_schema(
        tx: &Transaction,
        entity: EntitySchemaId,
        definition: ASD,
    ) -> AttributeSchemaId {
        let new_attribute = AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                entity,
                name: definition.name,
                quantity: definition.quantity,
                attr_type: CreateAttributeType::SimpleAttributeType(definition.attr_type),
            },
        )
        .expect("Failed to create attribute");

        new_attribute.id
    }

    pub struct RSD {
        name: String,
        quantity: Quantity,
    }

    impl Default for RSD {
        fn default() -> Self {
            Self {
                name: "Child".to_string(),
                quantity: Quantity::Required,
            }
        }
    }

    impl RSD {
        #[allow(unused)]
        pub fn name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }

        #[allow(unused)]
        pub fn quantity(mut self, quantity: Quantity) -> Self {
            self.quantity = quantity;
            self
        }
    }

    pub fn create_reference_schema(
        tx: &Transaction,
        parent: EntitySchemaId,
        child: EntitySchemaId,
        data: RSD,
    ) -> AttributeSchemaId {
        {
            let tx: &Transaction = &tx;
            let data = CreateAttributeSchema {
                    entity: parent,
                    name: data.name,
                    quantity: data.quantity,
                    attr_type: CreateAttributeType::CreateReferenceAttribute(
                        CreateReferenceAttribute { id: child },
                    ),
                };
            let reference = data.attr_type.get_ref();

            let new_attribute = AttributeSchema {
                id: AttributeSchemaId::new(),
                name: data.name,
                quantity: data.quantity,
                attr_type: data.attr_type.get_full(tx).unwrap(),
            };

            tx.execute(
                "INSERT INTO attribute_schema (id, entity, name, type, reference, quantity) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    &new_attribute.id,
                    data.entity,
                    &new_attribute.name,
                    &data.attr_type,
                    &reference,
                    &new_attribute.quantity
                ),
            ).unwrap();

            new_attribute
        }
        .id
    }

    pub fn assert_string_key(result: &Map<String, Value>, attr: AttributeSchemaId, expected: &str) {
        let val = result.get(&attr.to_string()).unwrap();

        assert_eq!(val, &Value::String(expected.to_string()));
    }
}
