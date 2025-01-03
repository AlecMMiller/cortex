pub mod entity;
pub mod schema;

#[derive(Debug, Type, thiserror::Error)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error("{0}")]
    Rusqlite(
        #[serde(skip)]
        #[from]
        rusqlite::Error,
    ),
    #[error("{0}")]
    R2D2(
        #[serde(skip)]
        #[from]
        r2d2::Error,
    ),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
