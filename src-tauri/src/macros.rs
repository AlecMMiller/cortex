pub mod macros {

    macro_rules! create_id {
        (
            $id_name:ident
        ) => {
            #[derive(Clone, Hash, Debug, PartialEq, Eq, specta::Type)]
            pub struct $id_name(#[specta(type = String)] Vec<u8>);

            impl std::fmt::Display for $id_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    let uuid = uuid::Uuid::from_slice(&self.0).unwrap();
                    let s = uuid.to_string();
                    write!(f, "{s}")
                }
            }

            impl TryFrom<&str> for $id_name {
                type Error = &'static str;

                fn try_from(s: &str) -> Result<$id_name, Self::Error> {
                    let uuid = uuid::Uuid::parse_str(&s).unwrap();
                    Ok($id_name {
                        0: uuid.as_bytes().to_vec(),
                    })
                }
            }

            impl rusqlite::ToSql for $id_name {
                fn to_sql(
                    &self,
                ) -> std::result::Result<rusqlite::types::ToSqlOutput<'_>, rusqlite::Error> {
                    Ok(rusqlite::types::ToSqlOutput::from(self.0.clone()))
                }
            }

            impl rusqlite::types::FromSql for $id_name {
                fn column_result(
                    value: rusqlite::types::ValueRef<'_>,
                ) -> rusqlite::types::FromSqlResult<Self> {
                    Ok($id_name {
                        0: value.as_blob().map(<[u8]>::to_vec)?,
                    })
                }
            }

            impl $id_name {
                pub fn new() -> Self {
                    $id_name {
                        0: uuid::Uuid::new_v4().as_bytes().to_vec(),
                    }
                }

                #[allow(dead_code)]
                pub fn column_result_manual(
                    value: rusqlite::types::ValueRef<'_>,
                ) -> rusqlite::types::FromSqlResult<Self> {
                    Ok($id_name {
                        0: value.as_blob().map(<[u8]>::to_vec)?,
                    })
                }
            }

            impl serde::Serialize for $id_name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::ser::Serializer,
                {
                    // Convert to V4 uuid
                    let uuid = uuid::Uuid::from_slice(&self.0).unwrap();
                    let s = uuid.to_string();
                    serializer.serialize_str(&s)
                }
            }

            impl<'de> serde::Deserialize<'de> for $id_name {
                fn deserialize<D>(deserializer: D) -> Result<$id_name, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let uuid = uuid::Uuid::parse_str(&s).unwrap();
                    Ok($id_name {
                        0: uuid.as_bytes().to_vec(),
                    })
                }
            }
        };
    }

    pub(crate) use create_id;
}
