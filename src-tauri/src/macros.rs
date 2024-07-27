pub mod macros {
    macro_rules! create_id {
        (
            $id_name:ident
        ) => {
            #[derive(AsExpression, FromSqlRow, Debug, PartialEq, Eq, Serialize)]
            #[sql_type = "diesel::sql_types::Binary"]
            pub struct $id_name (Vec<u8>);

            impl FromSql<Binary, Sqlite> for $id_name {
                fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                    Ok($id_name {
                        0: Vec::from_sql(bytes)?,
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
                        0: uuid::Uuid::new_v4().as_bytes().to_vec()
                    }
                }
            }
        };
    }

    pub(crate) use create_id;
}
