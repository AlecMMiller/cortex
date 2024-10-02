use tantivy::TantivyError;

pub mod notes;
pub mod settings;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("Tantivy stuff")]
    Tantivy(#[from] TantivyError),
}

// we must also implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
