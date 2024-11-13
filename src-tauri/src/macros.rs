pub mod macros {

    macro_rules! create_id {
        (
            $id_name:ident
        ) => {
            #[derive(Hash, AsExpression, FromSqlRow, Debug, PartialEq, Eq, specta::Type)]
            #[diesel(sql_type = diesel::sql_types::Binary)]
            pub struct $id_name(#[specta(type = String)] Vec<u8>);

            impl FromSql<Binary, Sqlite> for $id_name {
                fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                    Ok($id_name {
                        0: Vec::from_sql(bytes)?,
                    })
                }
            }

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

            impl ToSql<Binary, Sqlite> for $id_name {
                fn to_sql<'a>(&'a self, out: &mut Output<'a, '_, Sqlite>) -> serialize::Result {
                    ToSql::<Binary, Sqlite>::to_sql(&self.0, out)
                }
            }

            impl $id_name {
                pub fn new() -> Self {
                    $id_name {
                        0: uuid::Uuid::new_v4().as_bytes().to_vec(),
                    }
                }
            }

            impl Serialize for $id_name {
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

            impl<'de> Deserialize<'de> for $id_name {
                fn deserialize<D>(deserializer: D) -> Result<$id_name, D::Error>
                where
                    D: Deserializer<'de>,
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
