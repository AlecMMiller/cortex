#[cfg(test)]
pub mod test_util {
    use rusqlite::{Connection, Transaction};
    use serde_json::{Map, Value};

    use crate::{
        database::{migration::migrate, New},
        models::{
            attribute_schema::{
                AttributeSchema, AttributeSchemaId, CreateAttributeSchema, Quantity,
            },
            attribute_type::{CreateAttributeType, CreateReferenceAttribute, SimpleAttributeType},
            entity_schema::{CreateEntitySchema, EntitySchema, EntitySchemaId},
        },
        utils::get_timestamp,
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

    impl ESD {
        pub fn create(self, tx: &Transaction) -> EntitySchemaId {
            let schema = EntitySchema::new(&tx, CreateEntitySchema { name: self.name })
                .expect("Unable to create entity");

            schema.id
        }

        pub fn name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }

        pub fn create_default(tx: &Transaction) -> EntitySchemaId {
            let def = Self::default();
            def.create(&tx)
        }
    }

    pub struct ASD {
        name: String,
        attr_type: SimpleAttributeType,
        quantity: Quantity,
    }

    impl ASD {
        pub fn create(self, tx: &Transaction, entity: &EntitySchemaId) -> AttributeSchemaId {
            let new_attribute = AttributeSchema::new(
                &tx,
                CreateAttributeSchema {
                    entity: entity.clone(),
                    name: self.name,
                    quantity: self.quantity,
                    attr_type: CreateAttributeType::Simple(self.attr_type),
                },
            )
            .expect("Failed to create attribute");

            new_attribute.id
        }

        pub fn create_default(tx: &Transaction, entity: &EntitySchemaId) -> AttributeSchemaId {
            let def = Self::default();
            def.create(&tx, &entity)
        }

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

        pub fn create_default(
            tx: &Transaction,
            parent: &EntitySchemaId,
            child: &EntitySchemaId,
        ) -> AttributeSchemaId {
            let attr = Self::default();
            attr.create(tx, parent, child)
        }

        pub fn create(
            self,
            tx: &Transaction,
            parent: &EntitySchemaId,
            child: &EntitySchemaId,
        ) -> AttributeSchemaId {
            {
            let tx: &Transaction = &tx;
            let data = CreateAttributeSchema {
                    entity: parent.clone(),
                    name: self.name,
                    quantity: self.quantity,
                    attr_type: CreateAttributeType::Reference(
                        CreateReferenceAttribute { id: child.clone() },
                    ),
                };
            let reference = data.attr_type.get_ref();

            let new_attribute = AttributeSchema {
                id: AttributeSchemaId::new(),
                name: data.name,
                quantity: data.quantity,
                attr_type: data.attr_type.get_full(tx).unwrap(),
            };

            let created_at = get_timestamp();

            tx.execute(
                "INSERT INTO attribute_schema (id, entity, name, type, reference, quantity, created, updated) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
                (
                    &new_attribute.id,
                    data.entity,
                    &new_attribute.name,
                    &data.attr_type,
                    &reference,
                    &new_attribute.quantity,
                    created_at
                ),
            ).unwrap();

            new_attribute
        }
        .id
        }
    }

    pub fn assert_string_key(result: &Map<String, Value>, attr: AttributeSchemaId, expected: &str) {
        let val = result.get(&attr.to_string()).unwrap();

        assert_eq!(val, &Value::String(expected.to_string()));
    }
}
